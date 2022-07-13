pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
fn fibonacci1(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

const INF: f64 = 1000000.0;
// mod aabb;
// mod aarect;
// mod example_macro;
// mod hittable;
// mod material;
// mod onb;
// mod pdf;
// mod perlin;
// mod ray;
// mod rtweekend;
// mod run;
// mod scence;
// mod staticscence;
// mod texture;
// mod vec3;
// include!("aarect.rs");
// include!("aabb.rs");
// include!("camera.rs");
// include!("hittable.rs");
// include!("onb.rs");
// include!("material.rs");
// include!("scence.rs");
// include!("staticscence.rs");
// include!("pdf.rs");
// include!("ray.rs");
// include!("texture.rs");
// include!("vec3.rs");
use crate::aarect::{StaticXzRect, XzRect};
use crate::camera;
use crate::hittable::{Hittable, HittableList, StaticHittable, StaticHittableList};
use crate::material::{Lambertian, StaticLambertian};
use crate::pdf::{HittablePdf, MixturePdf, Pdf, StaticHittablePdf, StaticMixturePdf};
pub use crate::ray::Ray;
use crate::scence::{
    cornell_box, cornell_box_rabbit, cornell_smoke, earth, final_book2_scence,
    my_scence_ball_world, my_untimately, my_world, obj, obj_with_texture, random_sence,
    simple_light, two_berlin_spheres, two_spheres,
};
use crate::staticscence::{
    static_bvh_random_scence, static_cornell_box, static_earth, static_random_sence,
    two_texture_static,
};
use crate::texture::StaticBaseColor;
pub use crate::vec3::Vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
use std::f64::consts::PI;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

fn degree_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        _time0: f64,
        _time1: f64,
    ) -> Self {
        let theta = degree_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let view_heigth = 2.0 * h;
        let view_width = aspect_ratio * view_heigth;
        let w = Vec3::unit(lookfrom - lookat);
        let u = Vec3::unit(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let horizontemp = u * view_width * focus_dist;
        let verticaltemp = v * view_heigth * focus_dist;

        // let ratio: f64 = 16.0 / 9.0;
        // let view_heigth: f64 = 2.0;
        // let view_width = (view_heigth * ratio) as f64;
        Self {
            origin: Vec3 {
                x: lookfrom.x,
                y: lookfrom.y,
                z: lookfrom.z,
            },

            horizontal: Vec3 {
                x: horizontemp.x,
                y: horizontemp.y,
                z: horizontemp.z,
            },

            vertical: Vec3 {
                x: verticaltemp.x,
                y: verticaltemp.y,
                z: verticaltemp.z,
            },
            lower_left_corner: Vec3 {
                x: lookfrom.x - horizontemp.x / 2.0 - verticaltemp.x / 2.0 - w.x * focus_dist,
                y: lookfrom.y - horizontemp.y / 2.0 - verticaltemp.y / 2.0 - w.y * focus_dist,
                z: lookfrom.z - horizontemp.z / 2.0 - verticaltemp.z / 2.0 - w.z * focus_dist,
            },
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0: _time0,
            time1: _time1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            range_random_double(self.time0, self.time1),
        )
    }
}

pub(crate) fn random_doouble() -> f64 {
    rand::thread_rng().gen_range(1..101) as f64 / 102.0
}

pub(crate) fn range_random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random_doouble()
}

#[allow(clippy::needless_return)]
pub(crate) fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    return x;
}

#[allow(clippy::needless_return)]
#[allow(dead_code)]
fn color(
    x: Ray,
    background: Vec3,
    world: &HittableList,
    lights: &Arc<dyn Hittable + Send>,
    dep: u32,
) -> Vec3 {
    if dep == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Option::Some(_rec) = world.hit(x, 0.001, INF) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
        let emitted = _rec.mat_ptr.emitted(&_rec, _rec.u, _rec.v, &_rec.p);
        let mut pdf_val = 0.0;
        let mut aldedo = Vec3::zero();
        let scatterrecord =
            _rec.mat_ptr
                .scatter(&x, &_rec, &mut aldedo, &mut scattered, &mut pdf_val);

        if scatterrecord.isget {
            if scatterrecord.is_specular {
                return scatterrecord.attenuation
                    * color(
                        scatterrecord.specular_ray,
                        background,
                        world,
                        lights,
                        dep - 1,
                    );
            }

            let lightptr = Arc::new(HittablePdf::new(lights.clone(), &_rec.p));
            let p = MixturePdf::new(lightptr, scatterrecord.pdf_ptr);
            scattered = Ray::new(_rec.p, p.generate(), x.tm);

            pdf_val = p.value(&scattered.dic);

            let mm = emitted
                + scatterrecord.attenuation
                    * _rec.mat_ptr.scattering_odf(&x, &_rec, &scattered)
                    * color(scattered, background, world, lights, dep - 1)
                    / pdf_val;

            return mm;
        }
        emitted
    } else {
        background
    }
}

#[allow(dead_code)]
pub(crate) fn run() {
    let is_ci = match std::env::var("CI") {
        Ok(x) => x == "true",
        Err(_) => false,
    };
    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (16, 8) };
    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );
    let (tx, rx) = channel();
    let pool = ThreadPool::new(n_workers);
    //image
    let mut ratio: f64 = 1.0;
    // let image_width = 400 as u32;
    let mut image_width = 600_u32;
    let mut image_heigth = (image_width as f64 / ratio) as u32;
    let sample_per_pixel = 20; //ought to be 100  可以做的更大比如500//
    let max_depth = 50; //an bo modifyed to lessen the time
    let mut backgroud = Vec3::new(0.0, 0.0, 0.0);
    let mut lookfrom = Vec3::new(278.0, 278.0, -800.0); //13 2 3
    let mut lookat = Vec3::new(278.0, 278.0, 0.0);
    let mut vfov = 40.0;
    //choose picture you want
    let number = 1;
    let mut world = HittableList { objects: vec![] };
    match number {
        1 => {
            world = two_spheres();
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            vfov = 20.0;
        }
        2 => {
            world = random_sence();
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            vfov = 20.0;
            ratio = 3.0 / 2.0;

            image_width = 1200_u32;
            image_heigth = (image_width as f64 / ratio) as u32;
        }
        3 => {
            world = two_berlin_spheres();
            vfov = 20.0;
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
        }
        4 => {
            world = earth();
            vfov = 20.0;
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
        }
        5 => world = simple_light(),
        6 => {
            world = cornell_box();
            backgroud = Vec3::new(0.0, 0.0, 0.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
        }
        7 => world = cornell_smoke(),
        8 => {
            world = final_book2_scence();
            lookat = Vec3::new(278.0, 278.0, 0.0);
            lookfrom = Vec3::new(478.0, 278.0, -600.0); //todo
            vfov = 40.0;
            ratio = 1.0;
            image_width = 800_u32;
            image_heigth = (image_width as f64 / ratio) as u32;
        }
        9 => {
            world = my_scence_ball_world();
            lookat = Vec3::new(0.0, 0.0, 0.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 0.0);
            vfov = 50.0;
        }
        10 => {
            world = obj();
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(60.0, 20.0, 30.0);
            lookat = Vec3::zero();
        }
        11 => world = obj_with_texture(),
        12 => {
            world = cornell_box_rabbit();
            backgroud = Vec3::new(0.0, 0.0, 0.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
        }
        13 => {
            world = my_world();
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(70.0, 0.0, 0.0);
            lookat = Vec3::zero();
            image_width = 1600_u32;
            ratio = 1.0 / 0.618;
            image_heigth = (image_width as f64 / ratio) as u32;
            vfov = 62.0;
        }
        14 => {
            println!("begin build scence");
            world = my_untimately();
            println!("finish build scence");

            backgroud = Vec3::new(0.7, 0.8, 1.0);
            lookat = Vec3::new(500.0, 278.0, 0.0);
            lookfrom = Vec3::new(500.0, 278.0, -800.0);
            image_width = 1600_u32;
            ratio = 1.0 / 0.618;
            image_heigth = (image_width as f64 / ratio) as u32;
        }
        _ => println!("you are wrong !! please choose the wonderful world you want to see again."),
    }
    let mut lightworld: HittableList = HittableList { objects: vec![] };
    let light1: Arc<dyn Hittable + Send> = Arc::new(XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(Lambertian::new(Vec3::zero())),
    ));
    // let tmp: Arc<dyn Hittable + Send> = Arc::new(Sphere::new(
    //     Vec3::zero(),
    //     Vec3::zero(),
    //     0.0,
    //     Vec3::new(190.0, 90.0, 190.0),
    //     90.0,
    //     Arc::new(Lambertian::new(Vec3::zero())),
    // ));
    lightworld.add(light1);
    // lightworld.add(tmp);
    let lights: Arc<dyn Hittable + Send> = Arc::new(lightworld);

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0; //ought to be 2
    let cam = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    println!("init bar");
    let bar = ProgressBar::new(16);
    let world_inthread = Arc::new(world);
    println!("we start run");
    for i in 0..n_jobs {
        println!("i :{}", i);
        let tx = tx.clone();
        let worldptr = world_inthread.clone();
        let lightsptr = lights.clone();
        pool.execute(move || {
            let row_begin = image_heigth as usize * i / n_jobs;
            let row_end = image_heigth as usize * (i + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(image_width, render_height as u32);
            for x in 0..image_width {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let pixel = img.get_pixel_mut(x, img_y as u32);

                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..sample_per_pixel {
                        let u = (x as f64 + random_doouble()) / (image_width - 1) as f64;
                        let v = (image_heigth as f64 - y as f64 + random_doouble())
                            / (image_heigth - 1) as f64;
                        let r = cam.get_ray(u, v);
                        pixel_color += color(r, backgroud, &worldptr, &lightsptr, max_depth);
                    }
                    if pixel_color.x.is_nan() {
                        pixel_color.x = 0.0;
                    }
                    if pixel_color.y.is_nan() {
                        pixel_color.y = 0.0;
                    }
                    if pixel_color.z.is_nan() {
                        pixel_color.z = 0.0;
                    }
                    let scale = 1.0 / sample_per_pixel as f64;
                    let aaa = (pixel_color.x * scale).sqrt();
                    let aaa1 = 256_f64 * clamp(aaa, 0.0, 0.999);
                    let bbb = (pixel_color.y * scale).sqrt();
                    let bbb1 = 256_f64 * clamp(bbb, 0.0, 0.999);
                    let ccc = (pixel_color.z * scale).sqrt();
                    let ccc1 = 256_f64 * clamp(ccc, 0.0, 0.999);
                    *pixel = image::Rgb([aaa1 as u8, bbb1 as u8, ccc1 as u8]);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }
    let mut img: RgbImage = ImageBuffer::new(image_width, image_heigth);
    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_width {
                let row = row as u32;
                let idx = idx as u32;
                *img.get_pixel_mut(col, row) = *data.get_pixel(col, idx);
            }
        }
        bar.inc(1);
    }
    img.save("output/test.png").unwrap();
    bar.finish();
}

#[allow(dead_code)]
#[allow(clippy::needless_return)]
fn staticcolor<T: StaticHittable>(
    x: Ray,
    background: Vec3,
    world: &StaticHittableList,
    lights: &T,
    dep: u32,
) -> Vec3 {
    if dep == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Option::Some(_rec) = world.hit(x, 0.001, INF) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
        let emitted = _rec.mat_ptr.emitted(&_rec, _rec.u, _rec.v, &_rec.p);
        let mut pdf_val = 0.0;
        let mut aldedo = Vec3::zero();
        let scatterrecord =
            _rec.mat_ptr
                .scatter(&x, &_rec, &mut aldedo, &mut scattered, &mut pdf_val);

        if scatterrecord.isget {
            if scatterrecord.is_specular {
                return scatterrecord.attenuation
                    * staticcolor(
                        scatterrecord.specular_ray,
                        background,
                        world,
                        lights,
                        dep - 1,
                    );
            }

            let lightptr = StaticHittablePdf::new(lights, &_rec.p);
            let p = StaticMixturePdf::new(&lightptr, &scatterrecord.pdf_ptr);
            scattered = Ray::new(_rec.p, p.generate(), x.tm);

            pdf_val = p.value(&scattered.dic);

            let mm = emitted
                + scatterrecord.attenuation
                    * _rec.mat_ptr.scattering_odf(&x, &_rec, &scattered)
                    * staticcolor(scattered, background, world, lights, dep - 1)
                    / pdf_val;

            return mm;
        }
        emitted
    } else {
        background
    }
}

#[allow(dead_code)]
pub fn runstatic() {
    let is_ci = match std::env::var("CI") {
        Ok(x) => x == "true",
        Err(_) => false,
    };
    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (16, 8) };
    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );
    let (tx, rx) = channel();
    let pool = ThreadPool::new(n_workers);
    //image
    let mut ratio: f64 = 1.0;
    // let image_width = 400 as u32;
    let mut image_width = 600_u32;
    let mut image_heigth = (image_width as f64 / ratio) as u32;
    let sample_per_pixel = 5; //ought to be 100  可以做的更大比如500//
    let max_depth = 50; //an bo modifyed to lessen the time
    let mut backgroud = Vec3::new(0.0, 0.0, 0.0);
    let mut lookfrom = Vec3::new(278.0, 278.0, -800.0); //13 2 3
    let mut lookat = Vec3::new(278.0, 278.0, 0.0);
    let mut vfov = 40.0;
    let number = 5;
    let mut world = StaticHittableList { objects: vec![] };
    match number {
        1 => {
            world = two_texture_static();
            vfov = 20.0;
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
        }
        2 => {
            world = static_random_sence();
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            vfov = 20.0;
            ratio = 3.0 / 2.0;

            image_width = 1200_u32;
            image_heigth = (image_width as f64 / ratio) as u32;
        }
        3 => {
            world = static_earth();
            vfov = 20.0;
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
        }
        4 => {
            world = static_cornell_box();
            backgroud = Vec3::new(0.0, 0.0, 0.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
        }
        5 => {
            world = static_bvh_random_scence();
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            vfov = 20.0;
            ratio = 3.0 / 2.0;
            image_width = 1200_u32;
            image_heigth = (image_width as f64 / ratio) as u32;
        }

        _ => println!("you are wrong !! please choose the wonderful world you want to see again."),
    }

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0; //ought to be 2
    let cam = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    let bar = ProgressBar::new(16);
    let world_inthread = Arc::new(world);
    for i in 0..n_jobs {
        println!("yyy");
        let tx = tx.clone();
        let worldptr = world_inthread.clone();

        pool.execute(move || {
            let mut lightworld: StaticHittableList = StaticHittableList { objects: vec![] };
            let light1: Arc<dyn StaticHittable + Send> = Arc::new(StaticXzRect::new(
                213.0,
                343.0,
                227.0,
                332.0,
                554.0,
                StaticLambertian::<StaticBaseColor>::new(Vec3::zero()),
            ));
            // let tmp: Arc<dyn StaticHittable + Send> = Arc::new(StaticSphere::new(
            //     Vec3::zero(),
            //     Vec3::zero(),
            //     0.0,
            //     Vec3::new(190.0, 90.0, 190.0),
            //     90.0,
            //     StaticLambertian::<StaticBaseColor>::new(Vec3::zero()),
            // ));
            lightworld.add(light1);
            //lightworld.add(tmp);
            let lightin = lightworld;

            let row_begin = image_heigth as usize * i / n_jobs;
            let row_end = image_heigth as usize * (i + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(image_width, render_height as u32);
            for x in 0..image_width {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let pixel = img.get_pixel_mut(x, img_y as u32);

                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..sample_per_pixel {
                        let u = (x as f64 + random_doouble()) / (image_width - 1) as f64;
                        let v = (image_heigth as f64 - y as f64 + random_doouble())
                            / (image_heigth - 1) as f64;
                        let r = cam.get_ray(u, v);
                        pixel_color += staticcolor::<StaticHittableList>(
                            r, backgroud, &worldptr, &lightin, max_depth,
                        );
                    }
                    if pixel_color.x.is_nan() {
                        pixel_color.x = 0.0;
                    }
                    if pixel_color.y.is_nan() {
                        pixel_color.y = 0.0;
                    }
                    if pixel_color.z.is_nan() {
                        pixel_color.z = 0.0;
                    }
                    let scale = 1.0 / sample_per_pixel as f64;
                    let aaa = (pixel_color.x * scale).sqrt();
                    let aaa1 = 256_f64 * clamp(aaa, 0.0, 0.999);
                    let bbb = (pixel_color.y * scale).sqrt();
                    let bbb1 = 256_f64 * clamp(bbb, 0.0, 0.999);
                    let ccc = (pixel_color.z * scale).sqrt();
                    let ccc1 = 256_f64 * clamp(ccc, 0.0, 0.999);
                    *pixel = image::Rgb([aaa1 as u8, bbb1 as u8, ccc1 as u8]);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }
    let mut img: RgbImage = ImageBuffer::new(image_width, image_heigth);
    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_width {
                let row = row as u32;
                let idx = idx as u32;
                *img.get_pixel_mut(col, row) = *data.get_pixel(col, idx);
            }
        }
        bar.inc(1);
    }
    img.save("output/test.png").unwrap();
    bar.finish();
}
