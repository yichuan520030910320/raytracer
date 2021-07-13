#[allow(clippy::float_cmp)]
mod vec3;
mod ray;


use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;
pub use crate::ray::Ray;
fn hit_sphere(center:Vec3,radius:f64,r:Ray)->f64{
    let oc=r.ori-center;
    let a=Vec3::dot(r.dic,r.dic);
    let b=Vec3::dot(r.dic,oc)*2.0;
    let c=Vec3::dot(oc,oc)-radius*radius;
   let discriminant=( b*b-(4 as f64)*a*c)as f64;
    if discriminant<0.0 {-1.0 }
    else { (-b-discriminant.sqrt())/(2.0*a) }

}

fn color(x: Ray) -> Vec3 {
    let mut t=hit_sphere(Vec3::new(0.0,0.0,-1.0),0.5,x);
    if t>0.0 {
        let N=Vec3::unit(Ray::at(&x,t)-Vec3::new(0.0,0.0,-1.0));
      Vec3::new(N.x+1.0,N.y+1.0,N.z+1.0)*0.5
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
            let output:Vec3=color(r);

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
