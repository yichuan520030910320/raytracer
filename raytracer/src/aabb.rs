use crate::run::{Ray, Vec3};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Aabb {
    pub minimun: Vec3,
    pub maximum: Vec3,
}

#[allow(clippy::needless_return)]
fn fmin1(a: f64, b: f64) -> f64 {
    if a < b {
        return a;
    }
    return b;
}

#[allow(clippy::needless_return)]
fn fmax1(a: f64, b: f64) -> f64 {
    if a < b {
        return b;
    }
    return a;
}

#[allow(clippy::needless_return)]
impl Aabb {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            minimun: a,
            maximum: b,
        }
    }
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..2 {
            if a == 0 {
                let t0: f64;
                let t1: f64;
                if (self.minimun.x - r.ori.x) / r.dic.x < (self.maximum.x - r.ori.x) / r.dic.x {
                    t0 = (self.minimun.x - r.ori.x) / r.dic.x;
                    t1 = (self.maximum.x - r.ori.x) / r.dic.x;
                } else {
                    t0 = (self.maximum.x - r.ori.x) / r.dic.x;
                    t1 = (self.minimun.x - r.ori.x) / r.dic.x;
                }
                if t0 < t_min {} else {
                    t_min = t0;
                }
                if t1 < t_max {
                    t_max = t1;
                }
                if t_max <= t_min {
                    return false;
                }
            } else if a == 1 {
                let t0: f64;
                let t1: f64;
                if (self.minimun.y - r.ori.y) / r.dic.y < (self.maximum.y - r.ori.y) / r.dic.y {
                    t0 = (self.minimun.y - r.ori.y) / r.dic.y;
                    t1 = (self.maximum.y - r.ori.y) / r.dic.y;
                } else {
                    t0 = (self.maximum.y - r.ori.y) / r.dic.y;
                    t1 = (self.minimun.y - r.ori.y) / r.dic.y;
                }
                if t0 < t_min {} else {
                    t_min = t0;
                }
                if t1 < t_max {
                    t_max = t1;
                }
                if t_max <= t_min {
                    return false;
                }
            } else if a == 2 {
                let t0: f64;
                let t1: f64;
                if (self.minimun.z - r.ori.z) / r.dic.z < (self.maximum.z - r.ori.z) / r.dic.z {
                    t0 = (self.minimun.z - r.ori.z) / r.dic.z;
                    t1 = (self.maximum.z - r.ori.z) / r.dic.z;
                } else {
                    t0 = (self.maximum.z - r.ori.z) / r.dic.z;
                    t1 = (self.minimun.z - r.ori.z) / r.dic.z;
                }
                if t0 < t_min {} else {
                    t_min = t0;
                }
                if t1 < t_max {
                    t_max = t1;
                }
                if t_max <= t_min {
                    return false;
                }
            }
        }

        true
    }
    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Self {
        let small = Vec3::new(
            fmin1(box0.minimun.x, box1.minimun.x),
            fmin1(box0.minimun.y, box1.minimun.y),
            fmin1(box0.minimun.z, box1.minimun.z),
        );
        let big = Vec3::new(
            fmax1(box0.maximum.x, box1.maximum.x),
            fmax1(box0.maximum.y, box1.maximum.y),
            fmax1(box0.maximum.z, box1.maximum.z),
        );
        Aabb::new(small, big)
    }
}
