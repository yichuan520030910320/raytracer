use crate::aabb::Aabb;
use crate::hittable::{Hitrecord, Hittable, Material};
use crate::{range_random_double, Ray, Vec3};
use std::f64::INFINITY;
use std::sync::Arc;

pub fn maxnum1(a: f64, b: f64, c: f64) -> f64 {
    if a < b {
        if c < b {
            return b;
        } else {
            return c;
        }
    } else if a < c {
        return c;
    } else {
        return a;
    }
}

pub fn mainnum1(a: f64, b: f64, c: f64) -> f64 {
    if a < b {
        if c < a {
            return c;
        } else {
            return a;
        }
    } else if b < c {
        return b;
    } else {
        return c;
    }
}

pub struct XyRect {
    pub(crate) mp: Arc<dyn Material>,
    pub(crate) x0: f64,
    pub(crate) x1: f64,
    pub(crate) y0: f64,
    pub(crate) y1: f64,
    pub(crate) k: f64,
}

impl XyRect {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let t = (self.k - r.ori.z) / r.dic.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.ori.x + t * r.dic.x;
        let y = r.ori.y + t * r.dic.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let ourward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct Triangel {
    pub(crate) mp: Arc<dyn Material>,
    pub a1: Vec3,
    pub a2: Vec3,
    pub a3: Vec3,
}

unsafe impl Send for Triangel {}

unsafe impl Sync for Triangel {}

impl Triangel {
    pub fn new(_a1: Vec3, _a2: Vec3, _a3: Vec3, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            a1: _a1,
            a2: _a2,
            a3: _a3,
        }
    }
}

impl Hittable for Triangel {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let dirct1 = self.a2 - self.a1;
        let dirct2 = self.a3 - self.a1;
        let n = Vec3::cross(dirct1, dirct2);
        let b_a = self.a1 - r.ori;
        let t = Vec3::dot(n, b_a) / Vec3::dot(n, r.dic);
        //inspired by https://blog.csdn.net/wuwangrun/article/details/8188665
        if t < t_min || t > t_max {
            return None;
        }
        let hit = r.at(t);
        if Vec3::sameside(self.a1, self.a2, self.a3, hit)
            && Vec3::sameside(self.a2, self.a3, self.a1, hit)
            && Vec3::sameside(self.a3, self.a1, self.a2, hit)
        {
            //use the method 2 in https://www.cnblogs.com/graphics/archive/2010/08/05/1793393.html
            let mut rec = Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());
            rec.p = r.at(t);
            // let a1 = self.a1.x - self.a2.x;
            // let b1 = self.a1.x - self.a3.x;
            // let c1 = self.a1.x - hit.x;
            // let a2 = self.a1.y - self.a2.y;
            // let b2 = self.a1.y - self.a3.y;
            // let c2 = self.a1.y - hit.y;
            // rec.u=(c1*b2-b1*c2)/(a1*b2-b1*a2);
            // rec.v=(a1*c2-a2*c1)/(a1*b2-b1*a2);//may change the order //use the most stupid way to solve the problem
            // //the silly way
            rec.t = t;
            let ourward_normal = n;
            rec.set_face_normal(&r, ourward_normal);
            rec.mat_ptr = self.mp.clone();
            Some(rec)
        } else {
            return None;
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        // let dirct1 = self.a2 - self.a1;
        // let dirct2 = self.a3 - self.a1;
        // let n = Vec3::cross(dirct1, dirct2);
        let ans1 = Vec3::new(
            mainnum1(self.a1.x, self.a2.x, self.a3.x),
            mainnum1(self.a1.y, self.a2.y, self.a3.y),
            mainnum1(self.a1.z, self.a2.z, self.a3.z),
        ) + Vec3::new(0.01, 0.01, 0.01);
        let ans2 = Vec3::new(
            maxnum1(self.a1.x, self.a2.x, self.a3.x),
            maxnum1(self.a1.y, self.a2.y, self.a3.y),
            maxnum1(self.a1.z, self.a2.z, self.a3.z),
        ) - Vec3::new(0.01, 0.01, 0.01);

        Some(Aabb::new(ans1, ans2))
    }
}

pub struct XzRect {
    pub(crate) mp: Arc<dyn Material>,
    pub(crate) x0: f64,
    pub(crate) x1: f64,
    pub(crate) z0: f64,
    pub(crate) z1: f64,
    pub(crate) k: f64,
}

unsafe impl Send for XzRect {}

unsafe impl Sync for XzRect {}

impl XzRect {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let t = (self.k - r.ori.y) / r.dic.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.ori.x + t * r.dic.x;
        let z = r.ori.z + t * r.dic.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let ourward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        return if let Option::Some(rec) = self.hit(Ray::new(*o, *v, 0.0), 0.001, INFINITY) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = rec.t * rec.t * v.squared_length();
            let cosine = Vec3::dot(*v, rec.normal).abs() / v.length();

            distance_squared / (cosine * area)
        } else {
            0.0
        };
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let randompoint = Vec3::new(
            range_random_double(self.x0, self.x1),
            self.k,
            range_random_double(self.z0, self.z1),
        );
        return randompoint - *o;
    }
}

pub struct YzRect {
    pub(crate) mp: Arc<dyn Material>,
    pub(crate) y0: f64,
    pub(crate) y1: f64,
    pub(crate) z0: f64,
    pub(crate) z1: f64,
    pub(crate) k: f64,
}

impl YzRect {
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let t = (self.k - r.ori.x) / r.dic.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.ori.y + t * r.dic.y;
        let z = r.ori.z + t * r.dic.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());

        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let ourward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
