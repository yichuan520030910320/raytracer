#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod hittable;
mod camera;
mod rtweekend;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;
pub use crate::ray::Ray;
use crate::hittable::{HittableList, Sphere, Hittable};
use std::sync::Arc;
use std::f32::INFINITY;
use rand::Rng;

//let secret_number = ;
fn random_doouble()->f64{
    rand::thread_rng().gen_range(1..101) as f64/102.0
}
fn range_random_double(min:f64,max:f64)->f64{
    min+(max-min)*random_doouble()
}
fn clamp(x:f64,min:f64,max:f64)->f64{
    if x<min { return min}
    else if x>max { return max}
    return x
}
fn hit_sphere(center:Vec3,radius:f64,r:Ray)->f64{
    let oc=r.ori-center;
    let a=Vec3::squared_length(&r.dic);
    let half_b=Vec3::dot(r.dic,oc);
    let c=Vec3::squared_length(&oc)-radius*radius;
   let discriminant=( half_b*half_b-a*c)as f64;
    if discriminant<0.0 {-1.0 }
    else { (-half_b-discriminant.sqrt())/(a) }

}

fn color(x: Ray,world:&HittableList,dep:u32) -> Vec3 {
    if dep<=0 {return Vec3::new(0.0,0.0,0.0) }
    let mut t=hit_sphere(Vec3::new(0.0,0.0,-1.0),0.5,x);

    if let Option::Some(_rec) = world.hit(x,0.001,INFINITY as f64) {
        color(Ray::new(_rec.p,_rec.p+_rec.normal+Vec3::random_in_unit_sphere()-_rec.p),world,dep-1)*0.5

     //  (   _rec.normal+Vec3::new(1.0,1.0,1.0))*0.5
    }
    else { let unit_dir: Vec3 = Vec3::unit((x.dic));
         t = 0.5 * ((unit_dir.y.clone() + 1.0) as f64);
        Vec3::new(1.0, 1.0, 1.0)*((1.0 - t)as f64 ) + Vec3::new(0.5, 0.7, 1.0) *t as f64 }

}

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);
    //image
    let ratio: f64 = 16.0 / 9.0;
    let image_width = 400 as u32;
    let image_heigth = (image_width as f64/ ratio) as u32;
    let sample_per_pixel=60;//ought to be 100
    let max_depth=50;//an bo modifyed to lessen the time

    //world
    let mut world=HittableList{
        objects:vec![],
    };
    let sph1 = Sphere{
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0
        },
        radius: 0.5
    };
    let sph2 = Sphere{
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0
        },
        radius: 100.0
    };
    world.add(
        Arc::new(sph2)
    );

    world.add(
        Arc::new(sph1)
    );

    //Camera
    let cam=camera::Camera::new();

    let view_heigth: f64 = 2.0;
    let view_width= (view_heigth*ratio)as f64;
    let focallength=1.0;
   let origin:Vec3=Vec3::new(0.0,0.0,0.0);
    let horizon:Vec3=Vec3::new(view_width as f64,0.0,0.0);
    let vertical:Vec3=Vec3::new(0.0,view_heigth,0.0);
    let leftcorner=origin-horizon/2.0-vertical/2.0-Vec3::new(0.0,0.0,focallength);

    let mut img: RgbImage = ImageBuffer::new(image_width, image_heigth);
    let bar = ProgressBar::new(1024);

    for j in 0..image_heigth {
        for i in 0..image_width
        {let pixel = img.get_pixel_mut(i, j);
            let mut pixel_color =Vec3::new(0.0, 0.0, 0.0);
            for s in 0..sample_per_pixel {
                let u=(i as f64+random_doouble())/ (image_width-1)as f64;
                let v=(image_heigth as f64-j as f64+random_doouble())/(image_heigth-1) as f64;
                let r=cam.get_ray(u,v);
                pixel_color+=color(r, &world,max_depth);
            }



            // let aa: f32 = ((i) as f32 / (image_width-1)as f32) as f32;
            // let bb: f32 = ((image_heigth-j) as f32 / (image_heigth-1)as f32) as f32;
            // let r=Ray::new(origin,leftcorner+horizon*aa as f64+vertical*bb as f64-origin);
            // let output:Vec3=color(r, &world);
            let scale=1.0/sample_per_pixel as f64;
            let aaa=(pixel_color.x*scale).sqrt();
            let aaa1=256 as f64*clamp(aaa,0.0,0.999);
            let bbb=(pixel_color.y*scale).sqrt();
            let bbb1=256 as f64*clamp(bbb,0.0,0.999);
            let ccc=(pixel_color.z*scale).sqrt();
            let ccc1=256 as f64*clamp(ccc,0.0,0.999);
            *pixel = image::Rgb([aaa1 as u8, bbb1 as u8, ccc1 as u8]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
