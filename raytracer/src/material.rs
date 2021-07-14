pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
pub use crate::hittable::Hitrecord;
use std::sync::Arc;



pub trait Material {
    fn scatter(&self,r_in:&Ray,rec:&Hitrecord,attenuation:&mut Vec3,scattered:&mut Ray)->bool;//attenuation是衰减的意思

}
#[derive(Clone, Debug, PartialEq,Copy)]
pub struct Lambertian {
    albedo:Vec3,
}
impl Lambertian{
    pub fn new(albedo:Vec3)->Self{
        Self{albedo}
    }
}
impl Material for Lambertian{

    fn scatter(&self, r_in: &Ray, rec: &Hitrecord, attenuation: &mut Vec3, mut scattered: &mut Ray) -> bool {
        let scatter_direction=rec.normal+Vec3::random_unit_vector();
        scattered.dic=scatter_direction;
        scattered.ori=rec.p;
        // scattered= &mut Ray::new(rec.p.clone(), scatter_direction.clone());
        attenuation.x=self.albedo.x;
        attenuation.y=self.albedo.y;
        attenuation.z=self.albedo.z;
       // attenuation= &self.albedo;
        true
    }
}
#[derive(Clone, Debug, PartialEq,Copy)]
pub struct Metal {
    albedo:Vec3,
}
impl Metal{
    pub fn new(albedo:Vec3)->Self{
        Self{albedo}
    }
}
impl Material for Metal{
    fn scatter(&self, r_in: &Ray, rec: &Hitrecord, attenuation: &mut Vec3, mut scattered: &mut Ray) -> bool {
        let reflected=Vec3::reflect(r_in.dic,rec.normal);
        scattered.dic=reflected;
        scattered.ori=rec.p;
         //scattered= &mut Ray::new(rec.p, reflected.clone());
        attenuation.x=self.albedo.x;
        attenuation.y=self.albedo.y;
        attenuation.z=self.albedo.z;
        return Vec3::dot(scattered.dic, rec.normal)>0.0;
    }
}