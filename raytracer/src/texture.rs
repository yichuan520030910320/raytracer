use crate::run::{clamp, Vec3};
use image::RgbImage;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct CheckerTexture {
    color_value: Vec3,
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

#[derive(Clone)]
pub struct BaseColor {
    color: Vec3,
}

impl BaseColor {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for BaseColor {
    fn value(&self, _: f64, _: f64, _: &Vec3) -> Vec3 {
        //println!("{:?}",self.color);

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
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        }
    }
}

#[derive(Clone)]
pub struct ObjTexture {
    pub u: f64,
    pub v: f64,
    pub img: RgbImage,
}

#[derive(Clone)]
pub struct ImageTexture {
    pub width: i32,
    pub height: i32,
    pub bytes_per_scanline: i32,
    pub img: RgbImage,
}

impl ObjTexture {
    #[allow(dead_code)]
    pub fn new(filename: &str, u: f64, v: f64) -> Self {
        Self {
            u,
            v,
            img: image::open(filename).expect("failed").to_rgb8(),
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl Texture for ObjTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        let mut i = (self.u * ((self.img.width()) as f64)) as i32;
        let mut j = (self.v * ((self.img.height()) as f64)) as i32;
        if i >= self.img.width() as i32 {
            i = self.img.width() as i32 - 1;
        }
        if j >= self.img.height() as i32 {
            j = self.img.height() as i32 - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i as u32, j as u32);
        //let pixel=(self.data)+j*self.bytes_per_scanline+i*BYTES_PER_PIXEL;
        // println!(
        //     //"rnm  vec {:?}",
        //     Vec3::new(
        //         color_scale * (pixel[0] as f64),
        //         color_scale * (pixel[1] as f64),
        //         color_scale * (pixel[2] as f64),
        //     )
        // );

        Vec3::new(
            color_scale * (pixel[0] as f64),
            color_scale * (pixel[1] as f64),
            color_scale * (pixel[2] as f64),
        )
    }
}

impl ImageTexture {
    #[allow(dead_code)]
    pub fn new(filename: &str) -> Self {
        Self {
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
            img: image::open(filename).expect("failed").to_rgb8(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Vec3) -> Vec3 {
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
        Vec3::new(
            color_scale * (pixel[0] as f64),
            color_scale * (pixel[1] as f64),
            color_scale * (pixel[2] as f64),
        )
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct StaticCheckerTexture<T1: Texture, T2: Texture> {
    color_value: Vec3,
    odd: T1,
    even: T2,
}

#[derive(Clone)]
pub struct StaticBaseColor {
    color: Vec3,
}

impl StaticBaseColor {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for StaticBaseColor {
    fn value(&self, _: f64, _: f64, _: &Vec3) -> Vec3 {
        self.color
    }
}

impl StaticCheckerTexture<StaticBaseColor, StaticBaseColor> {
    #[allow(dead_code)]
    pub fn new(_odd: Vec3, _even: Vec3) -> Self {
        Self {
            color_value: Vec3::zero(),
            odd: StaticBaseColor::new(_odd),
            even: StaticBaseColor::new(_even),
        }
    }
}

impl<T1: Texture, T2: Texture> Texture for StaticCheckerTexture<T1, T2> {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        }
    }
}

#[derive(Clone)]
pub struct StaticObjTexture {
    pub u: f64,
    pub v: f64,
    pub img: RgbImage,
}

#[derive(Clone)]
pub struct StaticImageTexture {
    pub width: i32,
    pub height: i32,
    pub bytes_per_scanline: i32,
    pub img: RgbImage,
}

impl StaticObjTexture {
    #[allow(dead_code)]
    pub fn new(filename: &str, u: f64, v: f64) -> Self {
        Self {
            u,
            v,
            img: image::open(filename).expect("failed").to_rgb8(),
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl Texture for StaticObjTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        let mut i = (self.u * ((self.img.width()) as f64)) as i32;
        let mut j = (self.v * ((self.img.height()) as f64)) as i32;
        if i >= self.img.width() as i32 {
            i = self.img.width() as i32 - 1;
        }
        if j >= self.img.height() as i32 {
            j = self.img.height() as i32 - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i as u32, j as u32);
        //let pixel=(self.data)+j*self.bytes_per_scanline+i*BYTES_PER_PIXEL;
        // println!(
        //     "rnm  vec {:?}",
        //     Vec3::new(
        //         color_scale * (pixel[0] as f64),
        //         color_scale * (pixel[1] as f64),
        //         color_scale * (pixel[2] as f64),
        //     )
        // );

        Vec3::new(
            color_scale * (pixel[0] as f64),
            color_scale * (pixel[1] as f64),
            color_scale * (pixel[2] as f64),
        )
    }
}

impl StaticImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
            img: image::open(filename).expect("failed").to_rgb8(),
        }
    }
}

impl Texture for StaticImageTexture {
    fn value(&self, u: f64, v: f64, _: &Vec3) -> Vec3 {
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
        Vec3::new(
            color_scale * (pixel[0] as f64),
            color_scale * (pixel[1] as f64),
            color_scale * (pixel[2] as f64),
        )
    }
}
