use crate::{clamp, clamp1, Vec3};
use image::{RgbImage, RgbaImage};
use std::ptr::{null, null_mut};
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct CheckerTexture {
    color_value: Vec3,
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

pub struct BaseColor {
    color: Vec3,
}

impl BaseColor {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for BaseColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.color
    }
}

impl CheckerTexture {
    pub fn new(_odd: Vec3, _even: Vec3) -> Self {
        Self {
            color_value: Vec3::zero(),
            odd: Arc::new(BaseColor::new(_odd)),
            even: Arc::new(BaseColor::new(_even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, &p);
        } else {
            return self.even.value(u, v, &p);
        }
    }
}

const BYTES_PER_PIXEL: i32 = 3;

pub struct ImageTexture {
    pub width: i32,
    pub height: i32,
    pub bytes_per_scanline: i32,
    pub img: RgbImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
            img: image::open(filename).expect("failed").to_rgb(),
        }
        //todo
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        //         let u = clamp(u, 0.0, 1.0);
        //         let v = clamp(v, 0.0, 1.0);
        //         let x = clamp1(((u * self.img.width()as f64) as u32), 0, (self.img.width() - 1));
        //         let y = clamp1(((v * self.img.height() as f64) as u32), 0, (self.img.height() - 1));
        // //这边精度问题比较关键，数据类型要弄好
        //
        //         let color_scale = 1.0 / 255.0;
        //         let pixel = self.img.get_pixel(x , (self.img.height() )  - (y));
        //         return Vec3::new(color_scale * (pixel[0] as f64), color_scale * (pixel[1] as f64), color_scale * (pixel[2] as f64));

        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * ((self.img.width()) as f64)) as i32;
        let mut j = (v * ((self.img.height()) as f64)) as i32;

        if i >= self.img.width() as i32 {
            i = self.img.width() as i32 - 1;
        }
        if j >= self.img.height() as i32 {
            j = self.img.height() as i32 - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i as u32, j as u32);
        //let pixel=(self.data)+j*self.bytes_per_scanline+i*BYTES_PER_PIXEL;
        return Vec3::new(
            color_scale * (pixel[0] as f64),
            color_scale * (pixel[1] as f64),
            color_scale * (pixel[2] as f64),
        );
    }
}
