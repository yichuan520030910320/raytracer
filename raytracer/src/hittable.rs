pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Hitrecord {
    pub p: Vec3,
    //交点
    pub normal: Vec3,
    //法向量
    pub t: f64,
    //距离
    pub front_face: bool,//正面还是反面
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord>;
}//相当于一个基类 在列表里面会去看是谁将它实例化（如圆等图形）

impl Hitrecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool) -> Self { Self { p, normal, t, front_face } }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.dic, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}


pub struct Sphere {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(p: Vec3, normal: Vec3, t: f64, center: Vec3, radius: f64) -> Self { Self { p, normal, t, center, radius } }
}

//实例化trait在圆中
impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let oc = r.ori - self.center;
        let a = Vec3::squared_length(&r.dic);
        let half_b = Vec3::dot(r.dic, oc);
        let c = Vec3::squared_length(&oc) - self.radius * self.radius;
        let discriminant = (half_b * half_b - a * c) as f64;
        if discriminant < 0.0 {
           return None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None
                }
            }
            let mut rec = Hitrecord {
                t: 0.0,
                p: Vec3::zero(),
                normal: Vec3::zero(),
                front_face: false,
            };

            rec.t = root;
            rec.p = Ray::at(&r, rec.t);
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(&r, outward_normal);
            Some(rec)
        }
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,

//todo
//传出bool值可以用引用传递，先完善hittable 和add 函数
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let mut rec: Option<Hitrecord> = Option::None;
        let mut closet_so_far: f64 = t_max;
        for object in &self.objects {
            if let Option::Some(_rec) = object.hit(r, t_min, closet_so_far) {
                rec = Option::Some(_rec);

                closet_so_far = _rec.t;
            }
        }
        rec
    }
}