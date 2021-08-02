use crate::hittable::{Hittable, StaticHittable};
use crate::onb::Onb;
use crate::run::{random_doouble, Vec3};
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
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        let tempvec = Vec3::random_cosine_direction();
        self.uvw.local(tempvec.x, tempvec.y, tempvec.z)
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
    #[allow(dead_code)]
    pub fn new(p: Arc<dyn Hittable>, orgin: &Vec3) -> Self {
        Self { o: *orgin, ptr: p }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

pub struct MixturePdf {
    p0: Arc<dyn Pdf>,
    p1: Arc<dyn Pdf>,
}

impl MixturePdf {
    #[allow(dead_code)]
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p0, p1 }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.p0.value(direction) * 0.5 + self.p1.value(direction) * 0.5
    }

    fn generate(&self) -> Vec3 {
        if random_doouble() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
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
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::zero()
    }
}

//Static

pub struct StaticCosinePdf {
    uvw: Onb,
}

impl Pdf for StaticCosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = Vec3::dot(direction.unit(), self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        let tempvec = Vec3::random_cosine_direction();
        self.uvw.local(tempvec.x, tempvec.y, tempvec.z)
    }
}

impl StaticCosinePdf {
    #[allow(dead_code)]
    pub fn new(w: &Vec3) -> Self {
        let ans = Onb::build_from(w);
        Self { uvw: ans }
    }
}

pub struct StaticHittablePdf<'a, T: StaticHittable> {
    o: Vec3,
    ptr: &'a T,
}

impl<'a, T: StaticHittable> StaticHittablePdf<'a, T> {
    pub fn new(p: &'a T, orgin: &Vec3) -> Self {
        Self { o: *orgin, ptr: p }
    }
}

impl<'a, T: StaticHittable> Pdf for StaticHittablePdf<'a, T> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

pub struct StaticMixturePdf<'a, T1: Pdf, T2: Pdf> {
    p0: &'a T1,
    p1: &'a T2,
}

impl<'a, T1: Pdf, T2: Pdf> StaticMixturePdf<'a, T1, T2> {
    pub fn new(p0: &'a T1, p1: &'a T2) -> Self {
        Self { p0, p1 }
    }
}

impl<'a, T1: Pdf, T2: Pdf> Pdf for StaticMixturePdf<'a, T1, T2> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.p0.value(direction) * 0.5 + self.p1.value(direction) * 0.5
    }

    fn generate(&self) -> Vec3 {
        if random_doouble() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}

pub struct StaticNoPdf {
    pub val: f64,
}

impl StaticNoPdf {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { val: 0.0 }
    }
}

impl Pdf for StaticNoPdf {
    fn value(&self, _: &Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::zero()
    }
}
