use crate::Vec3;
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
        Self {
            color
        }
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
        } else { return self.even.value(u, v, &p); }
    }
}