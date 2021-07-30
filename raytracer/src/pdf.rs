use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::{random_doouble, Vec3};
use std::f64::consts::PI;
use std::sync::Arc;

pub trait Pdf: Send + Sync {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = Vec3::dot(direction.unit(), self.uvw.w());
        return if cosine <= 0.0 { 0.0 } else { cosine / PI };
    }

    fn generate(&self) -> Vec3 {
        let tempvec = Vec3::random_cosine_direction();
        return self.uvw.local(tempvec.x, tempvec.y, tempvec.z);
    }
}
impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        let ans = Onb::build_from(w);
        Self { uvw: ans }
    }
}
pub struct HittablePdf {
    o: Vec3,
    ptr: Arc<dyn Hittable>,
}
impl HittablePdf {
    pub fn new(p: Arc<dyn Hittable>, orgin: &Vec3) -> Self {
        Self { o: *orgin, ptr: p }
    }
}
impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        return self.ptr.pdf_value(&self.o, direction);
    }

    fn generate(&self) -> Vec3 {
        return self.ptr.random(&self.o);
    }
}
pub struct MixturePdf {
    p0: Arc<dyn Pdf>,
    p1: Arc<dyn Pdf>,
}
impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p0, p1 }
    }
}
impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        return self.p0.value(direction) * 0.5 + self.p1.value(direction) * 0.5;
    }

    fn generate(&self) -> Vec3 {
        return if random_doouble() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        };
    }
}
pub struct NoPdf {
    pub val: f64,
}

impl NoPdf {
    pub fn new() -> Self {
        Self { val: 0.0 }
    }
}

impl Pdf for NoPdf {
    fn value(&self, _: &Vec3) -> f64 {
        return 0.0;
    }
    fn generate(&self) -> Vec3 {
        return Vec3::zero();
    }
}
