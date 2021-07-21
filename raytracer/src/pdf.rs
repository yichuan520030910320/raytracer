use crate::Vec3;
use crate::onb::Onb;
use std::f64::consts::PI;
use std::sync::Arc;
use crate::hittable::{Hitrecord, Hittable};

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
            return 0.0;
        } else { return cosine / PI; }
    }

    fn generate(&self) -> Vec3 {
        let tempvec = Vec3::random_cosine_direction();
        return self.uvw.local(tempvec.x, tempvec.y, tempvec.z);
    }
}
impl CosinePdf{
    pub fn new(w:&Vec3)->Self{
        let ans=Onb::build_from(w);
        Self{
            uvw:ans,
        }
    }
}
pub struct HittablePdf{
    o:Vec3,
    ptr:Arc<dyn Hittable>,
}
impl HittablePdf{
    pub fn new(p:Arc<dyn Hittable>,orgin:&Vec3)->Self{
       Self{
           o: *orgin,
           ptr: p
       }
    }
}
impl Pdf for HittablePdf{
    fn value(&self, direction: &Vec3) -> f64 {
        return self.ptr.pdf_value(&self.o, direction);
    }

    fn generate(&self) -> Vec3 {
      return self.ptr.random(&self.o);
    }
}