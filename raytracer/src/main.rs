mod aabb;
mod aarect;
mod camera;
mod hittable;
mod material;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rtweekend;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;

const INF: f64 = 1000000.0;

use crate::aarect::{XyRect, XzRect, YzRect, Triangel};
use crate::hittable::{Box1, BvhNode, ConstantMedium, Hittable, HittableList, MovingSphere, RotateY, Sphere, Translate, RotateZ};
use crate::material::{Dielectric, DiffuseLight, FlipFace, Lambertian, Metal};
use crate::pdf::{HittablePdf, MixturePdf, Pdf};
use crate::perlin::NoiseTexture;
pub use crate::ray::Ray;
use crate::texture::{CheckerTexture, ImageTexture};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;
pub use vec3::Vec3;
use tobj;


fn random_doouble() -> f64 {
    rand::thread_rng().gen_range(1..101) as f64 / 102.0
}

fn range_random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random_doouble()
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    return x;
}

fn color(
    x: Ray,
    background: Vec3,
    world: &HittableList,
    lights: &Arc<dyn Hittable + Send>,
    dep: u32,
) -> Vec3 {
    if dep <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    return if let Option::Some(_rec) = world.hit(x, 0.001, INF) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
        let emitted = _rec.mat_ptr.emitted(&_rec, _rec.u, _rec.v, &_rec.p);
        let mut pdf_val = 0.0;
        let mut aldedo = Vec3::zero();
        let scatterrecord = _rec
            .mat_ptr
            .scatter(&x, &_rec, &mut aldedo, &mut scattered, &mut pdf_val);


        if scatterrecord.isget
        {
            if scatterrecord.is_specular {
                return scatterrecord.attenuation * color(scatterrecord.specular_ray, background, world, lights, dep - 1);
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
    };
}

fn main() {
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
    let ratio: f64 = 1.0;
    // let image_width = 400 as u32;
    let image_width = 800 as u32;
    let image_heigth = (image_width as f64 / ratio) as u32;
    let sample_per_pixel = 1; //ought to be 100  可以做的更大比如500//todo
    let max_depth = 50; //an bo modifyed to lessen the time
    let number = 10;
    let mut world = HittableList { objects: vec![] };
    match number {
        1 => world = two_spheres(),
        2 => world = random_sence(),
        3 => world = two_berlin_spheres(),
        4 => world = earth(),
        5 => world = simple_light(),
        6 => world = cornell_box(),
        7 => world = cornell_smoke(),
        8 => world = final_book2_scence(),
        9 => world = my_scence_ball_world(),
        10 => world = obj(),
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
    let tmp: Arc<dyn Hittable + Send> = Arc::new(Sphere::new(Vec3::zero(), Vec3::zero(), 0.0, Vec3::new(190.0, 90.0, 190.0), 90.0, Arc::new(Lambertian::new(Vec3::zero()))));
    lightworld.add(light1);
    lightworld.add(tmp);
    let lights: Arc<dyn Hittable + Send> = Arc::new(lightworld);
    // let world=earth();
    //  let world=two_spheres();//todo
    // let backgroud=Vec3::new(0.7,0.8,1.0);
    // let backgroud = Vec3::new(0.0, 0.0, 0.0);
    // let backgroud = Vec3::new(0.7, 0.8, 1.0);
    let backgroud = Vec3::new(0.7, 0.8, 1.0);
    //Camera
    //let lookfrom = Vec3::new(278.0, 278.0, -800.0); //13 2 3
    //let lookfrom = Vec3::new(478.0, 278.0, -600.0); //13 2 3
    let lookfrom = Vec3::new(13.0, 9.0, 0.0);//13 2 3

    let lookfrom = Vec3::new(-1300.0, 2500.0, 2000.0);//13 2 3
   // let lookfrom = Vec3::new(3.0, 2.0, 0.0);//13 2 3
    // let lookfrom=Vec3::new(26.0,3.0,6.0);//13 2 3
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    //let lookat=Vec3::new(0.0,2.0,0.0);
    //let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0; //ought to be 2
    let cam = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        90.0,
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
                    if pixel_color.x != pixel_color.x {
                        pixel_color.x = 0.0;
                    }
                    if pixel_color.y != pixel_color.y {
                        pixel_color.y = 0.0;
                    }
                    if pixel_color.z != pixel_color.z {
                        pixel_color.z = 0.0;
                    }
                    let scale = 1.0 / sample_per_pixel as f64;
                    let aaa = (pixel_color.x * scale).sqrt();
                    let aaa1 = 256 as f64 * clamp(aaa, 0.0, 0.999);
                    let bbb = (pixel_color.y * scale).sqrt();
                    let bbb1 = 256 as f64 * clamp(bbb, 0.0, 0.999);
                    let ccc = (pixel_color.z * scale).sqrt();
                    let ccc1 = 256 as f64 * clamp(ccc, 0.0, 0.999);
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

fn two_spheres() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let below = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: -10.0,
            z: 0.0,
        },
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)), //todo
    };
    world.add(Arc::new(below));
    let checker1 = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    let above = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        },
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian::new1(checker1)), //todo
    };
    world.add(Arc::new(above));
    return world;
}

fn random_sence() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let ground = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)), //todo
    };
    world.add(Arc::new(ground));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_doouble();
            let center = Vec3::new(
                a as f64 + 0.9 * random_doouble(),
                0.2,
                b as f64 + 0.9 * random_doouble(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let center2 = center + Vec3::new(0.0, range_random_double(0.0, 0.5), 0.0);
                    let temp = MovingSphere {
                        center0: center,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat_ptr: Arc::new(Lambertian::new(albedo)),
                    };
                    world.add(Arc::new(temp));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::randomrange(0.5, 1.0);
                    let fuzz = range_random_double(0.0, 0.5);
                    let temp = Sphere {
                        p: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        normal: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        t: 0.0,
                        center: Vec3 {
                            x: center.x.clone(),
                            y: center.y.clone(),
                            z: center.z.clone(),
                        },
                        radius: 0.2,
                        mat_ptr: Arc::new(Metal::new(albedo, fuzz)),
                    };
                    world.add(Arc::new(temp));
                } else {
                    let temp = Sphere {
                        p: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        normal: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        t: 0.0,
                        center: Vec3 {
                            x: center.x.clone(),
                            y: center.y.clone(),
                            z: center.z.clone(),
                        },
                        radius: 0.2,
                        mat_ptr: Arc::new(Dielectric::new(1.5)),
                    };

                    world.add(Arc::new(temp));
                }
            }
        }
    }
    let material1 = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    };
    world.add(Arc::new(material1));
    let material2 = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    };
    world.add(Arc::new(material2));

    let material3 = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    };
    world.add(Arc::new(material3));
    return world;
}

fn two_berlin_spheres() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let checker = Arc::new(NoiseTexture::new(4.0));
    let below = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)), //todo
    };
    world.add(Arc::new(below));

    let checker1 = Arc::new(NoiseTexture::new(4.0));
    let above = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Arc::new(Lambertian::new1(checker1)), //todo
    };
    world.add(Arc::new(above));

    //todo
    return world;
}

fn earth() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let checker = Arc::new(ImageTexture::new("earthmap.jpg"));
    let below = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)), //todo
    };
    world.add(Arc::new(below));
    return world;
}

fn simple_light() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let checker = Arc::new(NoiseTexture::new(4.0));
    let below = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)), //todo
    };
    world.add(Arc::new(below));

    let checker1 = Arc::new(NoiseTexture::new(4.0));
    let above = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Arc::new(Lambertian::new1(checker1)), //todo
    };
    world.add(Arc::new(above));
    let difflight1 = XyRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(4.0, 4.0, 4.0))),
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
    };
    world.add(Arc::new(difflight1));
    let light = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: 6.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Arc::new(DiffuseLight::new(Vec3::new(4.0, 4.0, 4.0))),
    };
    world.add(Arc::new(light));

    //todo
    return world;
}

fn cornell_box() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let red = YzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05))),
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(red));

    let green = YzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15))),

        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(green));
    let white1 = XzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(white1));
    let white2 = XzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(white2));

    let white3 = XyRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(white3));

    // whitebox4 = Box1::new( &Vec3::new(130.0, 0.0, 65.0), &Vec3::new(295.0, 165.0, 230.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    //
    // world.add(
    //     Arc::new(whitebox4)
    // );

    // let mut whitebox1: Arc<dyn Hittable> = Arc::new(Box1::new(
    //     &Vec3::new(0.0, 0.0, 0.0),
    //     &Vec3::new(165.0, 330.0, 165.0),
    //     Arc::new((Lambertian::new(Vec3::new(0.73, 0.73, 0.73)))),
    // ));
    // whitebox1 = Arc::new(RotateY::new(whitebox1, 15.0));
    // whitebox1 = Arc::new(Translate::new(whitebox1, Vec3::new(265.0, 0.0, 295.0)));
    // world.add(whitebox1);


    let mut whitebox1: Arc<dyn Hittable> = Arc::new(Box1::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 330.0, 165.0),
        Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0)),
    ));
    whitebox1 = Arc::new(RotateY::new(whitebox1, 15.0));
    whitebox1 = Arc::new(Translate::new(whitebox1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(whitebox1);

    let material1 = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 190.0,
            y: 90.0,
            z: 190.0,
        },
        radius: 90.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    };
    world.add(Arc::new(material1));

    // let mut whitebox2: Arc<dyn Hittable> = Arc::new(Box1::new(
    //     &Vec3::new(0.0, 0.0, 0.0),
    //     &Vec3::new(165.0, 165.0, 165.0),
    //     Arc::new((Lambertian::new(Vec3::new(0.73, 0.73, 0.73)))),
    // ));
    // whitebox2 = Arc::new(RotateY::new(whitebox2, -18.0));
    // whitebox2 = Arc::new(Translate::new(whitebox2, Vec3::new(130.0, 0.0, 65.0)));
    // world.add(whitebox2);

    // let whitebox5 = Box1::new( &Vec3::new(265.0, 0.0, 295.0), &Vec3::new(430.0, 330.0, 460.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    // world.add(
    //     Arc::new(whitebox5)
    // );

    let light1 = XzRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0))),
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
    };
    let light1_bonus = Arc::new(FlipFace::new(Arc::new(light1)));
    world.add(light1_bonus);
    return world;
}

fn cornell_smoke() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let red = YzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05))),
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(red));

    let green = YzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15))),

        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(green));
    let white1 = XzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(white1));
    let white2 = XzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(white2));
    let white3 = XyRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(white3));
    // whitebox4 = Box1::new( &Vec3::new(130.0, 0.0, 65.0), &Vec3::new(295.0, 165.0, 230.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    //
    // world.add(
    //     Arc::new(whitebox4)
    // );
    let mut whitebox1: Arc<dyn Hittable> = Arc::new(Box1::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 330.0, 165.0),
        Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),
    ));
    whitebox1 = Arc::new(RotateY::new(whitebox1, 15.0));
    whitebox1 = Arc::new(Translate::new(whitebox1, Vec3::new(265.0, 0.0, 295.0)));
    whitebox1 = Arc::new(ConstantMedium::new(
        whitebox1,
        0.01,
        Vec3::new(0.0, 0.0, 0.0),
    ));
    world.add(whitebox1);
    let mut whitebox2: Arc<dyn Hittable> = Arc::new(Box1::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 165.0, 165.0),
        Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),
    ));
    whitebox2 = Arc::new(RotateY::new(whitebox2, -18.0));
    whitebox2 = Arc::new(Translate::new(whitebox2, Vec3::new(130.0, 0.0, 65.0)));
    whitebox2 = Arc::new(ConstantMedium::new(
        whitebox2,
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
    ));
    world.add(whitebox2);
    // let whitebox5 = Box1::new( &Vec3::new(265.0, 0.0, 295.0), &Vec3::new(430.0, 330.0, 460.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    // world.add(
    //     Arc::new(whitebox5)
    // );
    let light1 = XzRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(7.0, 7.0, 7.0))),
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,
    };
    world.add(Arc::new(light1));
    return world;
}

fn final_book2_scence() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let light1 = XzRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(7.0, 7.0, 7.0))),
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,
    };
    world.add(Arc::new(light1));

    let mut boxes1 = HittableList { objects: vec![] };
    let boxes_per_sides = 20;
    for i in 0..boxes_per_sides {
        for j in 0..boxes_per_sides {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w as f64;
            let z0 = -1000.0 + j as f64 * w as f64;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = range_random_double(1.0, 101.0);
            let z1 = z0 + w;
            let whitebox4 = Box1::new(
                &Vec3::new(x0, y0, z0),
                &Vec3::new(x1, y1, z1),
                Arc::new(Lambertian::new(Vec3::new(0.48, 0.83, 0.53))),
            );
            boxes1.add(Arc::new(whitebox4));
        }
    }
    world.add(Arc::new(BvhNode::new(boxes1.objects, 0.0, 1.0)));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let temp = MovingSphere {
        center0: center1,
        center1: center2,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.1))),
    };
    world.add(Arc::new(temp));

    let tempmetal = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: 150.0,
            z: 145.0,
        },
        radius: 50.0,
        mat_ptr: Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
    };
    world.add(Arc::new(tempmetal));

    let tempdielectric = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 260.0,
            y: 150.0,
            z: 45.0,
        },
        radius: 50.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    };

    world.add(Arc::new(tempdielectric));

    let tempdielectric2 = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 360.0,
            y: 150.0,
            z: 145.0,
        },
        radius: 70.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    };

    world.add(Arc::new(tempdielectric2));
    let mut boudary: Arc<dyn Hittable> = Arc::new(Sphere::new(
        Vec3::zero(),
        Vec3::zero(),
        0.0,
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    boudary = Arc::new(ConstantMedium::new(boudary, 0.2, Vec3::new(0.2, 0.4, 0.9)));
    world.add(boudary);
    let mut boudary2: Arc<dyn Hittable> = Arc::new(Sphere::new(
        Vec3::zero(),
        Vec3::zero(),
        0.0,
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    boudary2 = Arc::new(ConstantMedium::new(
        boudary2,
        0.0001,
        Vec3::new(1.0, 1.0, 1.0),
    ));
    world.add(boudary2);
    let checker = Arc::new(ImageTexture::new("earthmap.jpg"));
    let below = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 400.0,
            y: 200.0,
            z: 400.0,
        },
        radius: 100.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)), //todo
    };
    world.add(Arc::new(below));
    let checker = Arc::new(NoiseTexture::new(0.1));
    let pertext = Sphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 220.0,
            y: 280.0,
            z: 300.0,
        },
        radius: 80.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)), //todo
    };
    world.add(Arc::new(pertext));
    let mut boxes2 = HittableList { objects: vec![] };
    for _ in 0..1000 {
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: Vec3::randomrange(0.0, 165.0),
            radius: 10.0,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))), //todo
        };
        boxes2.add(Arc::new(ground));
    }

    let allin = Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    world.add(allin);

    return world;
}

fn my_scence_ball_world() -> HittableList {
    let cam = Vec3::new(13.0, 2.0, 0.0);
    let mut world = HittableList { objects: vec![] };
    let light1 = XyRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(0.0, 0.0, -1.0))),
        x0: -30.0,
        x1: 30.0,
        y0: 0.0,

        k: 20.0,//高度
        y1: 6.0,
    };
    world.add(Arc::new(light1));

    let floortexture = Arc::new(ImageTexture::new("wondersky.jpg"));
    let floor = Sphere::new(Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }, Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }, 0.0, Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }, 100.0, Arc::new(Lambertian::new1(floortexture)));
    let terminalfloor = Arc::new(Translate::new(Arc::new(RotateZ::new(Arc::new(floor), 60.0)), Vec3::new(0.0, -100.0, 0.0)));

    world.add(terminalfloor);
    let mut boxes2 = HittableList { objects: vec![] };
    for _ in 0..2500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.01, 0.05);
        a.z = range_random_double(-50.0, 50.0);
        a.x = range_random_double(-50.0, 150.0);
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.05,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.3125, 0.0, 0.50))), //todo
        };
        boxes2.add(Arc::new(ground));
    }
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.00, 0.02);
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-10.0, 10.0);
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.05,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.3125, 0.0, 0.50))), //todo
        };
        boxes2.add(Arc::new(ground));
    }
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.00, 0.02);
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-1.0, 12.0);
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.05,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.0, 0.99, 0.0))), //green
        };
        boxes2.add(Arc::new(ground));
    }
    for _ in 0..600 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.01, 0.04);
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(1.0, 13.0);
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.03,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.99, 0.00, 0.0))), //red
        };
        boxes2.add(Arc::new(ground));
    }

    for _ in 0..1000 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.03, 0.04);
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-10.0, 14.0);
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.03,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.99, 0.41100, 0.7058))), //pink
        };
        boxes2.add(Arc::new(ground));
    }

    let sky1 = Arc::new(ImageTexture::new("sky.png"));
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.03, 0.04) + 0.005;
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-1.0, 14.0);
        let sky_sphere = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.04,
            mat_ptr: Arc::new(Lambertian::new1(sky1.clone())), //todo
        };
        boxes2.add(Arc::new(sky_sphere));
    }
    let redtexture = Arc::new(ImageTexture::new("red2.jpg"));
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.03, 0.04) + 2.305;
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-1.0, 14.0);
        let sky_sphere = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.04,
            mat_ptr: Arc::new(Lambertian::new1(redtexture.clone())), //todo
        };
        boxes2.add(Arc::new(sky_sphere));
    }

    let universetexture = Arc::new(ImageTexture::new("universal.jpg"));
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.03, 0.04) + 1.305;
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-1.0, 10.0);
        let sky_sphere = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.03,
            mat_ptr: Arc::new(Lambertian::new1(universetexture.clone())), //todo
        };
        boxes2.add(Arc::new(sky_sphere));
    }

    let universetexture = Arc::new(ImageTexture::new("universal2.jpg"));
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.03, 0.04) + 1.305 - 0.5;
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-1.0, 10.0);
        let sky_sphere = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.03,
            mat_ptr: Arc::new(Lambertian::new1(universetexture.clone())), //todo
        };
        boxes2.add(Arc::new(sky_sphere));
    }


    let allin = Arc::new(Translate::new(
        Arc::new(RotateZ::new(Arc::new(RotateY::new(
            Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
            0.0,
        )), 0.0)),
        Vec3::new(0.0, 0.0, 0.0),
    ));
    world.add(allin);


    let mut boxes_red = HittableList { objects: vec![] };
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.0, 2.0);
        a.z = range_random_double(-5.0, 5.0);
        a.x = range_random_double(-10.0, 9.0);
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.055,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.99, 0.27, 0.0))), //todo
        };
        boxes_red.add(Arc::new(ground));
    }
    let allinred = Arc::new(BvhNode::new(boxes_red.objects, 0.0, 1.0));
    world.add(allinred);


    let mut boxes_yellow = HittableList { objects: vec![] };
    for _ in 0..400 {
        let mut a = Vec3::zero();
        a.y = range_random_double(1.90, 2.50);
        a.z = range_random_double(-5.0, 5.0);
        a.x = range_random_double(-20.0, 6.0);
        if (a - cam).length() < 0.1 { continue; }
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.0592,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.99, 0.388, 0.27801))), //todo
        };
        boxes_yellow.add(Arc::new(ground));
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_yellow.objects, 0.0, 1.0));
    world.add(allinyellow);


    let mut boxes_pink = HittableList { objects: vec![] };
    for _ in 0..400 {
        let mut a = Vec3::zero();
        a.y = range_random_double(3.90, 4.50) - 0.5;
        a.z = range_random_double(-9.0, 9.0);
        a.x = range_random_double(-20.0, 9.0);
        if (a - cam).length() < 0.1 { continue; }
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.0592,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.901, 0.541, 0.727))), //todo
        };
        boxes_pink.add(Arc::new(ground));
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_pink.objects, 0.0, 1.0));
    world.add(allinyellow);


    let mut boxes_white = HittableList { objects: vec![] };
    for _ in 0..400 {
        let mut a = Vec3::zero();
        a.y = range_random_double(1.70, 2.90);
        a.z = range_random_double(-5.0, 5.0);
        a.x = range_random_double(-20.0, 11.0);
        if (a - cam).length() < 0.1 { continue; }
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.0792,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.99, 0.99, 0.99))), //todo
        };
        boxes_white.add(Arc::new(ground));
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_white.objects, 0.0, 1.0));
    world.add(allinyellow);

    let mut boxes_greenblue = HittableList { objects: vec![] };
    for i in -12..12 {
        for j in -12..12 {
            let mut a = Vec3::zero();
            a.y = range_random_double(2.8, 4.90);
            a.z = i as f64 * 0.5;
            a.x = j as f64 * 0.5;
            if (a - cam).length() < 0.1 { continue; }
            let ground = Sphere {
                p: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                normal: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                t: 0.0,
                center: a,
                radius: 0.0792,
                mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.529, 0.8078, 0.9215))), //todo
            };
            boxes_greenblue.add(Arc::new(ground));
        }
    }
    for i in -11..11 {
        for j in -11..11 {
            let mut a = Vec3::zero();
            a.y = range_random_double(1.9, 2.20);
            a.z = i as f64 * 1.0 + 0.1 + j as f64 * 0.2;
            a.x = j as f64 * 1.0 + 0.1;
            if (a - cam).length() < 0.1 { continue; }
            let ground = Sphere {
                p: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                normal: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                t: 0.0,
                center: a,
                radius: 0.0792,
                mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.529, 0.8078, 0.9215))), //todo
            };
            boxes_greenblue.add(Arc::new(ground));
        }
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_greenblue.objects.clone(), 0.0, 1.0));
    world.add(allinyellow);


    let mut boxes_green = HittableList { objects: vec![] };
    for _ in 0..200 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.50, 1.160);
        a.z = range_random_double(-5.0, 5.0);
        a.x = range_random_double(-20.0, 12.0);
        if (a - cam).length() < 0.1 { continue; }
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.0792,
            mat_ptr: Arc::new(Metal::new(Vec3::new(0.564, 0.933, 0.564), 0.1)), //todo
        };
        boxes_green.add(Arc::new(ground));
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_green.objects, 0.0, 1.0));
    world.add(allinyellow);

    let mut boxes_dark_green = HittableList { objects: vec![] };
    for _ in 0..600 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.05, 0.15);
        a.z = range_random_double(-5.0, 5.0);
        a.x = range_random_double(-20.0, 12.0);
        if (a - cam).length() < 0.1 { continue; }
        let ground = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            center: a,
            radius: 0.0392,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.0, 0.3933, 0.0))), //todo
        };
        boxes_dark_green.add(Arc::new(ground));
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_dark_green.objects, 0.0, 1.0));
    world.add(allinyellow);


//
    let mut tennis_unit = HittableList { objects: vec![] };
    let tennis = Arc::new(ImageTexture::new("tennis.png"));
    for i in -8..8 {
        if i == 0 { continue; }
        let xx = i as f64 * (1.7 / 8.0);
        let sky_sphere = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            radius: 0.1160,
            mat_ptr: Arc::new(Lambertian::new1(tennis.clone())), //todo
            center: Vec3 {
                x: 0.0,
                y: 3.0 - xx * xx,
                z: xx,
            },
        };
        tennis_unit.add(Arc::new(sky_sphere));//
    }

    let allteniss = Translate::new(Arc::new(RotateY::new(Arc::new(tennis_unit), 60.0)), Vec3::new(6.0, -0.05, -1.0));
    world.add(Arc::new(allteniss));


    let mut glass_unit = HittableList { objects: vec![] };
    for i in -8..8 {
        if i == 0 { continue; }
        let xx = i as f64 * (1.75 / 8.0);

        let sky_sphere = Sphere {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            radius: 0.0960,
            mat_ptr: Arc::new(Dielectric::new(1.5)), //todo
            center: Vec3 {
                x: 0.0,
                y: 2.0 - 0.667 * xx * xx,
                z: xx,
            },
        };
        glass_unit.add(Arc::new(sky_sphere));//
    }

    let allteniss = Translate::new(Arc::new(RotateY::new(Arc::new(glass_unit), -30.0)), Vec3::new(9.0, -0.05, 2.0));
    world.add(Arc::new(allteniss));
    return world;
}

fn obj() -> HittableList {
    let mut world = HittableList { objects: vec![] };


    let cornell_box = tobj::load_obj(
        "bunny.fine.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );
    assert!(cornell_box.is_ok());
    let rate = 10000.0;
    let (models, materials) = cornell_box.expect("Failed to load OBJ file");

// Materials might report a separate loading error if the MTL file wasn't found.
// If you don't need the materials, you can generate a default here and use that
// instead.
    let materials = materials.expect("Failed to load MTL file");


    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("model[{}].incidices: {}", i, mesh.indices.len() / 3);

        assert!(mesh.indices.len() % 3 == 0);

        let mut boxes2 = HittableList { objects: vec![] };
        for v in 0..mesh.indices.len() / 3 {


            // println!(
            //     "  indices  v[{}] = ({}, {}, {})",
            //     v,
            //     mesh.indices[3 * v],
            //     mesh.indices[3 * v + 1],
            //     mesh.indices[3 * v + 2]
            // );
            let x1 = mesh.indices[3 * v];
            let x2 = mesh.indices[3 * v + 1];
            let x3 = mesh.indices[3 * v + 2];

            let triange = Triangel::new(Vec3 {
                x: rate * mesh.positions[(3 * x1) as usize] as f64,
                y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
            }, Vec3 {
                x: rate * mesh.positions[(3 * x2) as usize] as f64,
                y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
            }, Vec3 {
                x: rate * mesh.positions[(3 * x3) as usize] as f64,
                y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
            }, Arc::new(Lambertian::new(Vec3::new(0.39, 0.7, 0.34))));
            boxes2.add(Arc::new(triange));
        }
        let allin = BvhNode::new(boxes2.objects, 0.0, 1.0);
        world.add(Arc::new(allin));

        // println!("model[{}].name = \'{}\'", i, m.name);
        // println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);
        //
        // println!(
        //     "Size of model[{}].face_arities: {}",
        //     i,
        //     mesh.face_arities.len()
        // );

        let mut next_face = 0;
        for f in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
            println!("    face[{}] = {:?}", f, face_indices);
            next_face = end;
        }

        // Normals and texture coordinates are also loaded, but not printed in this example
        // println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        //
        // assert!(mesh.positions.len() % 3 == 0);
        // for v in 0..mesh.positions.len() / 3 {
        //     println!(
        //         "    v[{}] = ({}, {}, {})",
        //         v,
        //         mesh.positions[3 * v],
        //         mesh.positions[3 * v + 1],
        //         mesh.positions[3 * v + 2]
        //     );
        // }
        //
        //
    }


    for (i, m) in materials.iter().enumerate() {
        println!("material[{}].name = \'{}\'", i, m.name);
        println!(
            "    material.Ka = ({}, {}, {})",
            m.ambient[0], m.ambient[1], m.ambient[2]
        );
        println!(
            "    material.Kd = ({}, {}, {})",
            m.diffuse[0], m.diffuse[1], m.diffuse[2]
        );
        println!(
            "    material.Ks = ({}, {}, {})",
            m.specular[0], m.specular[1], m.specular[2]
        );
        println!("    material.Ns = {}", m.shininess);
        println!("    material.d = {}", m.dissolve);
        println!("    material.map_Ka = {}", m.ambient_texture);
        println!("    material.map_Kd = {}", m.diffuse_texture);
        println!("    material.map_Ks = {}", m.specular_texture);
        println!("    material.map_Ns = {}", m.shininess_texture);
        println!("    material.map_Bump = {}", m.normal_texture);
        println!("    material.map_d = {}", m.dissolve_texture);

        for (k, v) in &m.unknown_param {
            println!("    material.{} = {}", k, v);
        }
    }

    return world;
}

fn obj_with_texture() -> HittableList {
    let mut world = HittableList { objects: vec![] };


    let cornell_box = tobj::load_obj(
        "patrick.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );
    assert!(cornell_box.is_ok());
    let rate = 10000.0;
    let (models, materials) = cornell_box.expect("Failed to load OBJ file");

// Materials might report a separate loading error if the MTL file wasn't found.
// If you don't need the materials, you can generate a default here and use that
// instead.
    let materials = materials.expect("Failed to load MTL file");


    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("model[{}].incidices: {}", i, mesh.indices.len() / 3);

        assert!(mesh.indices.len() % 3 == 0);

        let mut boxes2 = HittableList { objects: vec![] };
        // for v in 0..mesh.indices.len() / 3 {
        //
        //
        //     println!(
        //         "  indices  v[{}] = ({}, {}, {})",
        //         v,
        //         mesh.indices[3 * v],
        //         mesh.indices[3 * v + 1],
        //         mesh.indices[3 * v + 2]
        //     );
        //     let x1 = mesh.indices[3 * v];
        //     let x2 = mesh.indices[3 * v + 1];
        //     let x3 = mesh.indices[3 * v + 2];
        //
        //     let triange = Triangel::new(Vec3 {
        //         x: rate * mesh.positions[(3 * x1) as usize] as f64,
        //         y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
        //         z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
        //     }, Vec3 {
        //         x: rate * mesh.positions[(3 * x2) as usize] as f64,
        //         y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
        //         z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
        //     }, Vec3 {
        //         x: rate * mesh.positions[(3 * x3) as usize] as f64,
        //         y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
        //         z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
        //     }, Arc::new(Lambertian::new(Vec3::new(0.39, 0.7, 0.34))));
        //     boxes2.add(Arc::new(triange));
        // }
        // let allin = BvhNode::new(boxes2.objects, 0.0, 1.0);
        // world.add(Arc::new(allin));
        //
        // println!("model[{}].name = \'{}\'", i, m.name);
        // println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);
        //
        // println!(
        //     "Size of model[{}].face_arities: {}",
        //     i,
        //     mesh.face_arities.len()
        // );

        let mut next_face = 0;
        for f in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
            println!("    face[{}] = {:?}", f, face_indices);
            next_face = end;
        }

        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);

        assert!(mesh.positions.len() % 3 == 0);
        for v in 0..mesh.positions.len() / 3 {
            println!(
                "    v[{}] = ({}, {}, {})",
                v,
                mesh.positions[3 * v],
                mesh.positions[3 * v + 1],
                mesh.positions[3 * v + 2]
            );
        }


    }


    for (i, m) in materials.iter().enumerate() {
        println!("material[{}].name = \'{}\'", i, m.name);
        println!(
            "    material.Ka = ({}, {}, {})",
            m.ambient[0], m.ambient[1], m.ambient[2]
        );
        println!(
            "    material.Kd = ({}, {}, {})",
            m.diffuse[0], m.diffuse[1], m.diffuse[2]
        );
        println!(
            "    material.Ks = ({}, {}, {})",
            m.specular[0], m.specular[1], m.specular[2]
        );
        println!("    material.Ns = {}", m.shininess);
        println!("    material.d = {}", m.dissolve);
        println!("    material.map_Ka = {}", m.ambient_texture);
        println!("    material.map_Kd = {}", m.diffuse_texture);
        println!("    material.map_Ks = {}", m.specular_texture);
        println!("    material.map_Ns = {}", m.shininess_texture);
        println!("    material.map_Bump = {}", m.normal_texture);
        println!("    material.map_d = {}", m.dissolve_texture);

        for (k, v) in &m.unknown_param {
            println!("    material.{} = {}", k, v);
        }
    }

    return world;
}