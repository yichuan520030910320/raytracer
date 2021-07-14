#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod hittable;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;
pub use crate::ray::Ray;
use crate::hittable::{HittableList, Sphere, Hittable};
use std::sync::Arc;
use std::f32::INFINITY;

fn hit_sphere(center:Vec3,radius:f64,r:Ray)->f64{
    let oc=r.ori-center;
    let a=Vec3::squared_length(&r.dic);
    let half_b=Vec3::dot(r.dic,oc);
    let c=Vec3::squared_length(&oc)-radius*radius;
   let discriminant=( half_b*half_b-a*c)as f64;
    if discriminant<0.0 {-1.0 }
    else { (-half_b-discriminant.sqrt())/(a) }

}

fn color(x: Ray,world:&HittableList) -> Vec3 {
    let mut t=hit_sphere(Vec3::new(0.0,0.0,-1.0),0.5,x);
    if let Option::Some(_rec) = world.hit(x,0.0,INFINITY as f64) {
       (   _rec.normal+Vec3::new(1.0,1.0,1.0))*0.5
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

    //camera

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
        {
            let pixel = img.get_pixel_mut(i, j);
           // let color = (x) as u8;
            let aa: f32 = ((i) as f32 / (image_width-1)as f32) as f32;
            let bb: f32 = ((image_heigth-j) as f32 / (image_heigth-1)as f32) as f32;
            let r=Ray::new(origin,leftcorner+horizon*aa as f64+vertical*bb as f64-origin);
            let output:Vec3=color(r, &world);

            let cc: f32 = (0.25) as f32;
            let aa1: u8 = (255.99 * aa) as u8;
            let bb1: u8 = (255.99 * bb) as u8;
            let cc1: u8 = (255.99 * cc) as u8;
//
            *pixel = image::Rgb([(output.x*255.99)as u8, (output.y*255.99)as u8, (output.z*255.99)as u8]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
