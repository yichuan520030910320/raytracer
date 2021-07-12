#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;

fn main() {

    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);


    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);

    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            let color = (x ) as u8;
            let aa: f32 =( (x)as f32/255.0)as f32 ;
            let bb: f32 = ((512-y)as f32/255.0)as f32;
            let cc: f32 = (0.25)as f32;
            let aa1: u8 = (255.99*aa)as u8;
            let bb1: u8 = (255.99*bb)as u8;
            let cc1: u8 = (255.99*cc)as u8;
//
            *pixel = image::Rgb([aa1, bb1, cc1]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
