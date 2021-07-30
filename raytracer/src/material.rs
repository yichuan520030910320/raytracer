use crate::aabb::Aabb;
pub use crate::hittable::Hitrecord;
use crate::hittable::Hittable;
use crate::random_doouble;
pub use crate::ray::Ray;
use crate::texture::{BaseColor, Texture};
pub use crate::vec3::Vec3;
use std::f64::consts::PI;
use std::sync::Arc;
use crate::pdf::{Pdf, CosinePdf, NoPdf};
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

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Vec3,
    pub pdf_ptr: Arc<dyn Pdf>,
    pub isget: bool,
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> ScatterRecord;
    //attenuation是衰减的意思
    fn emitted(&self, _: &Hitrecord, _: f64, _: f64, _: &Vec3) -> Vec3 {
        return Vec3::zero();
    }
    fn scattering_odf(&self, _: &Ray, _: &Hitrecord, _: &Ray) -> f64 {
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
        _: &Ray,
        rec: &Hitrecord,
        _: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> ScatterRecord {

        ScatterRecord {
            specular_ray: Ray::new(Vec3::zero(), Vec3::zero(), 0.0),
            is_specular: false,
            attenuation: Vec3 {
                x: self.albedo.value(rec.u, rec.v, &rec.p).x,
                y: self.albedo.value(rec.u, rec.v, &rec.p).y,
                z: self.albedo.value(rec.u, rec.v, &rec.p).z,
            },
            pdf_ptr: Arc::new(CosinePdf::new(&rec.normal)),
            isget: true,
        }
    }
    fn scattering_odf(&self, _: &Ray, rec: &Hitrecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal, scattered.dic.clone().unit());
        return if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
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
        if fuzz < 1.0 {} else {
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
        _: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> ScatterRecord {
        let reflected = Vec3::reflect(r_in.dic, rec.normal);

        ScatterRecord {
            specular_ray: Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz, 0.0),
            is_specular: true,
            attenuation: self.albedo,
            pdf_ptr: Arc::new(NoPdf::new()),
            isget: true,
        }
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
        _: &mut Ray,
        _: &mut f64,
    ) -> ScatterRecord {
        let mut srec: ScatterRecord = ScatterRecord {
            specular_ray: Ray {
                ori: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                dic: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tm: 0.0,
            },
            is_specular: false,
            attenuation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            pdf_ptr: Arc::new(NoPdf::new()),
            isget: true,
        };
        srec.is_specular = true;
        srec.pdf_ptr = Arc::new(NoPdf::new());
        srec.attenuation = Vec3::new(1.0, 1.0, 1.0);
        srec.isget = true;

        // attenuation=Vec3::new(1.0,1.0,1.0);
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0; //glass dont absorb ray so the attenuation is constly 1

        let  etai_over_etat:f64;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx
        } else {
            etai_over_etat = self.ref_idx
        }


        let unit_direction = Vec3::unit(r_in.dic);
        let  cos_theta :f64;
        if Vec3::dot(-unit_direction, rec.normal) < 1.0 {
            cos_theta = Vec3::dot(-unit_direction, rec.normal)
        } else {
            cos_theta = 1.0
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);

            srec.specular_ray = Ray::new(rec.p, reflected, r_in.tm);
            return srec;
        }
        let reflect_pro = schlick(cos_theta, etai_over_etat);
        if random_doouble() < reflect_pro {
            let reflected = Vec3::reflect(unit_direction, rec.normal);

            srec.specular_ray = Ray::new(rec.p, reflected, r_in.tm);

            return srec;
        }

        let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        srec.specular_ray = Ray::new(rec.p, refracted, r_in.tm);
        return srec;
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
    fn scatter(
        &self,
        _: &Ray,
        _: &Hitrecord,
        _: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> ScatterRecord {
        ScatterRecord {
            specular_ray: Ray {
                ori: Vec3::zero(),
                dic: Vec3::zero(),
                tm: 0.0,
            },
            is_specular: false,
            attenuation: Vec3::zero(),
            pdf_ptr: Arc::new(NoPdf::new()),
            isget: false,
        }
    }
    fn emitted(&self, rec: &Hitrecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
        if rec.front_face {
            return self.emit.value(u, v, p);
        }

        return self.emit.value(u, v, p);
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
        _: &mut Ray,
        _: &mut f64,
    ) -> ScatterRecord {
        let temp = self.albedo.value(rec.u, rec.v, &rec.p);
        attenuation.x = temp.x;
        attenuation.y = temp.y;
        attenuation.z = temp.z;
        ScatterRecord {
            specular_ray: Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.tm),
            is_specular: false,
            attenuation: Vec3::new(temp.x, temp.y, temp.z),
            pdf_ptr: Arc::new(NoPdf::new()),
            isget: true,
        }
    }
    fn scattering_odf(&self, _: &Ray, rec: &Hitrecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal, scattered.dic.clone().unit());
        return if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

pub struct FlipFace {
    ptr: Arc<dyn Hittable>,
}

impl FlipFace {
    pub fn new(ptr: Arc<dyn Hittable>) -> Self {
        Self { ptr }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        if let Option::Some(mut rec) = self.ptr.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.ptr.bounding_box(time0, time1)
    }
}
