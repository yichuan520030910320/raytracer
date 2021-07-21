pub use crate::hittable::Hitrecord;
use crate::random_doouble;
pub use crate::ray::Ray;
use crate::texture::{BaseColor, Texture};
pub use crate::vec3::Vec3;
use std::sync::Arc;
use std::f64::consts::PI;
use crate::onb::Onb;
use std::collections::hash_map::Entry::Vacant;
use crate::hittable::Hittable;
use crate::aabb::Aabb;

const HALFNUM: f64 = 0.5;
fn schlick(cosin: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    return r0
        + (1.0 - r0)
            * (1.0 - cosin)
            * (1.0 - cosin)
            * (1.0 - cosin)
            * (1.0 - cosin)
            * (1.0 - cosin);
}

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        pdf:& mut f64,
    ) -> bool{

        // let uvw=Onb::build_from(&rec
        //     .normal);
        // let tempvec=Vec3::random_cosine_direction();
        // let direction=uvw.local(tempvec.x,tempvec.y,tempvec.z);
        // scattered.ori=rec.p;
        // scattered.dic=direction.unit();
        // scattered.tm=r_in.tm;




        return false;
    }
    //attenuation是衰减的意思
    fn emitted(&self,rec:&Hitrecord, u: f64, v: f64,p:&Vec3) -> Vec3 {
        let aa = u;
        let bb = v;
        let m = p.x;

        return Vec3::zero();
    }
     fn scattering_odf(&self,r_in:&Ray,rec:&Hitrecord,scattered:&Ray)->f64{
        return 0.0;
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo: Arc::new(BaseColor::new(albedo)),
        }
    }
    pub fn new1(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        mut scattered: &mut Ray,
        mut pdf: &mut f64,
    ) -> bool {
     let uvw=Onb::build_from(&rec.normal);

        let temp=Vec3::random_cosine_direction();
        let mut scatter_direction =uvw.local(temp.x,temp.y,temp.z);
        // if Vec3::near_zero(scatter_direction) {
        //     scatter_direction = rec.normal;
        // }
        scattered.ori = rec.p;
        scattered.dic = scatter_direction.unit();

        scattered.tm = r_in.tm;

        // scattered= &mut Ray::new(rec.p.clone(), scatter_direction.clone());
        attenuation.x = self.albedo.value(rec.u, rec.v, &rec.p).x;
        attenuation.y = self.albedo.value(rec.u, rec.v, &rec.p).y;
        attenuation.z = self.albedo.value(rec.u, rec.v, &rec.p).z;
       // let mut temp =  (HALFNUM/PI);
       //
       //  *pdf=  temp;
        *pdf=  (Vec3::dot(uvw.w(),scatter_direction) / PI);

        // attenuation= &self.albedo;
        true
    }
    fn scattering_odf(&self,_:&Ray,rec:&Hitrecord,scattered:&Ray)->f64{
        let cosine=Vec3::dot(rec.normal,scattered.dic.clone().unit());
        if cosine<0.0 {
            return 0.0;
        }
        else {
            return cosine/PI;
        }

    }


}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, mut fuzz: f64) -> Self {
        if fuzz < 1.0 {
        } else {
            fuzz = 1.0
        }
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        mut scattered: &mut Ray,
        mut pdf: &mut f64,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.dic, rec.normal);
        scattered.dic = reflected + Vec3::random_in_unit_sphere() * self.fuzz;
        scattered.ori = rec.p;
        scattered.tm = r_in.tm;
        //scattered= &mut Ray::new(rec.p, reflected.clone());
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        return Vec3::dot(scattered.dic, rec.normal) > 0.0;
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        mut pdf: &mut f64,
    ) -> bool {
        // attenuation=Vec3::new(1.0,1.0,1.0);
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0; //glass dont absorb ray so the attenuation is constly 1

        let mut etai_over_etat = 0.0;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx
        } else {
            etai_over_etat = self.ref_idx
        }

        // println!("{}", etai_over_etat);

        let unit_direction = Vec3::unit(r_in.dic);

        // let refracted=Vec3::refract(unit_direction,rec.normal,etai_over_etat);
        // scattered.ori=rec.p;
        // scattered.dic=refracted;
        // return  true;
        let mut cos_theta = 0.0;
        if Vec3::dot(-unit_direction, rec.normal) < 1.0 {
            cos_theta = Vec3::dot(-unit_direction, rec.normal)
        } else {
            cos_theta = 1.0
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            scattered.ori = rec.p;
            scattered.dic = reflected;
            scattered.tm = r_in.tm;
            return true;
        }
        let reflect_pro = schlick(cos_theta, etai_over_etat);
        if random_doouble() < reflect_pro {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            scattered.ori = rec.p;
            scattered.dic = reflected;
            scattered.tm = r_in.tm;
            return true;
        }

        let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        scattered.ori = rec.p;
        scattered.dic = refracted;
        scattered.tm = r_in.tm;
        return true;
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(c: Vec3) -> Self {
        Self {
            emit: Arc::new(BaseColor::new(c)),
        }
    }
    pub fn new1(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self,rec:&Hitrecord, u: f64, v: f64,p:&Vec3)  -> Vec3 {
        if rec.front_face {
            //println!("1");
             return self.emit.value(u, v, p);
        }
        //println!("0");
        return Vec3::zero();
    }
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        mut pdf: &mut f64,
    ) -> bool {
        return false;
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(c: Vec3) -> Self {
        Self {
            albedo: Arc::new(BaseColor::new(c)),
        }
    }
    pub fn new1(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        mut pdf: &mut f64,
    ) -> bool {
        scattered.ori = rec.p;
        scattered.dic = Vec3::random_in_unit_sphere();
        scattered.tm = r_in.tm;
        let temp = self.albedo.value(rec.u, rec.v, &rec.p);
        attenuation.x = temp.x;
        attenuation.y = temp.y;
        attenuation.z = temp.z;
        return true;
    }
}
pub struct FlipFace {
    ptr:Arc<dyn Hittable>,
}
impl FlipFace{
    pub fn new(ptr:Arc<dyn Hittable>)->Self{
        Self{ptr}
    }
}
impl Hittable for FlipFace{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
       if let Option::Some (mut rec)=self.ptr.hit(r, t_min, t_max){
           rec.front_face=!rec.front_face;
          return Some(rec);
       }
        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
       self.ptr.bounding_box(time0,time1)
    }
}