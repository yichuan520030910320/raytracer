pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
pub use crate::material::Material;
pub use crate::aabb::Aabb;
use std::sync::Arc;
use rand::Rng;
use std::thread::sleep;
use std::io::empty;
use std::f64::consts::PI;
use crate::aarect::{XyRect, XzRect, YzRect};

#[derive(Clone)]
pub struct Hitrecord {
    pub p: Vec3,
    //交点
    pub normal: Vec3,
    //法向量
    pub t: f64,
    pub u: f64,
    pub v: f64,
    //距离
    pub front_face: bool,
    //正面还是反面
    pub mat_ptr: Arc<dyn Material>,
}

fn random_doouble() -> f64 {
    rand::thread_rng().gen_range(1..101) as f64 / 102.0
}

fn range_random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random_doouble()
}

fn random_int(min: i32, max: i32) -> i32 {
    return range_random_double(min as f64, (max + 1) as f64) as i32;
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}//相当于一个基类 在列表里面会去看是谁将它实例化（如圆等图形）

impl Hitrecord {
    pub fn grt_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let temptheta = (-p.z) / p.x;

        let mut phi = (temptheta).atan();
        phi = phi + PI;
        *u = *&mut (phi / (2.0 * PI));
        *v = *&mut (theta / PI);
    }
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool, mat_ptr: Arc<dyn Material>) -> Self { Self { p, normal, t, u: 0.0, v: 0.0, front_face, mat_ptr } }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.dic, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(cen0: Vec3, cen1: Vec3, _time0: f64, _time1: f64, r: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: _time0,
            time1: _time1,
            radius: r,
            mat_ptr,
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        return self.center0 + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0));
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let oc = r.ori - MovingSphere::center(self, r.tm);
        let a = Vec3::squared_length(&r.dic);
        let half_b = Vec3::dot(r.dic, oc);
        let c = Vec3::squared_length(&oc) - self.radius * self.radius;
        let discriminant = (half_b * half_b - a * c) as f64;
        if discriminant < 0.0 {
            return None;
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }
            let mut rec = Hitrecord {
                t: 0.0,
                u: 0.0,
                p: Vec3::zero(),
                normal: Vec3::zero(),
                front_face: false,
                mat_ptr: self.mat_ptr.clone(),
                v: 0.0,
            };

            rec.t = root;
            rec.p = Ray::at(&r, rec.clone().t);
            let outward_normal = (rec.p - MovingSphere::center(self, r.tm)) / self.radius;
            rec.set_face_normal(&r, outward_normal);
            Some(rec)
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let output_box = Aabb::surrounding_box(box0, box1);
        return Some(output_box);
        //改成option
    }
}

pub struct Sphere {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}


impl Sphere {
    pub fn new(p: Vec3, normal: Vec3, t: f64, center: Vec3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self { Self { p, normal, t, center, radius, mat_ptr } }
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
            return None;
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }
            let mut rec = Hitrecord {
                t: 0.0,
                u: 0.0,
                p: Vec3::zero(),
                normal: Vec3::zero(),
                front_face: false,
                mat_ptr: self.mat_ptr.clone(),
                v: 0.0,
            };
            rec.t = root;
            rec.p = Ray::at(&r, rec.t);
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(&r, outward_normal);
            Hitrecord::grt_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
            Some(rec)
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let output = Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        return Some(output);
    }
}

pub struct Box1 {
    pub(crate) box_min: Vec3,
    pub(crate) box_max: Vec3,
    pub(crate) sides: HittableList,
}

impl Hittable for Box1 {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        return self.sides.hit(r, t_min, t_max);
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        return Option::from(Aabb::new(self.box_min, self.box_max));
    }
}

impl Box1 {
    pub fn new(&mut self, p0: &Vec3, p1: &Vec3, ptr: Arc<dyn Material>)->Self {
        self.box_min = *p0;
        self.box_max = *p1;
        let mut world = HittableList {
            objects: vec![],
        };
        let obj1 = XyRect {
            mp: ptr.clone(),
            x0: p0.x,
            x1: p1.x,
            y0: p0.y,
            y1: p1.y,
            k: p1.z,

        };
        let obj2 = XyRect {
            mp: ptr.clone(),
            x0: p0.x,
            x1: p1.x,
            y0: p0.y,
            y1: p1.y,
            k: p0.z,

        };
        let obj3 = XzRect {
            mp: ptr.clone(),
            x0: p0.x,
            x1: p1.x,
            z0: p0.z,
            z1: p1.z,
            k: p1.y,

        };
        let obj4 = XzRect {
            mp: ptr.clone(),
            x0: p0.x,
            x1: p1.x,
            z0: p0.z,
            z1: p1.z,
            k: p0.z,

        };
        let obj5 = YzRect {
            mp: ptr.clone(),
            y0: p0.y,
            y1: p1.y,
            z0: p0.z,
            z1: p1.z,
            k: p1.x,

        };
        let obj6 = YzRect {
            mp: ptr.clone(),
            y0: p0.y,
            y1: p1.y,
            z0: p0.z,
            z1: p1.z,
            k: p0.x,

        };
        world.add(
            Arc::new(obj1)
        );
        world.add(
            Arc::new(obj2)
        );
        world.add(
            Arc::new(obj3)
        );
        world.add(
            Arc::new(obj4)
        );
       world.add(
            Arc::new(obj5)
        );
        world.add(
            Arc::new(obj6)
        );
        return Self{
            box_min: self.box_min,
            box_max: self.box_max,
            sides: world
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
        let mut closet_so_far = t_max;
        for object in self.objects.iter() {
            if let Option::Some(_rec) = object.hit(r, t_min, closet_so_far) {
                rec = Option::Some(_rec.clone());
                closet_so_far = _rec.t;
            }
        }
        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() { return None; }
        let mut output = Aabb::new(Vec3::zero(), Vec3::zero());
        let tempbox = Aabb::new(Vec3::zero(), Vec3::zero());
        let mut first_box = true;
        for object in self.objects.iter() {
            if let Option::Some(tempbox) = object.bounding_box(time0, time1) {
                if first_box {
                    output = tempbox;
                } else {
                    output = Aabb::surrounding_box(output, tempbox);
                }
                first_box = false;
            } else { return None; }
        }
        Some(output)
    }
}

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub box1: Aabb,

}

impl BvhNode {
    pub fn bouding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let outout = self.box1.clone();
        return Some(outout);
    }
    pub fn new(
        src_objects: &mut Vec<Arc<dyn Hittable>>, span: i32, time0: f64, time1: f64,
    ) -> Self {
        let mut objects = src_objects;
        let mut rng = rand::thread_rng();
        let axis = rand::thread_rng().gen_range(0..3);
        //todo
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if span == 1 {
            left = objects.remove(0);
            right = left.clone();
        } else if span == 2 {
            left = objects.remove(0);
            right = objects.remove(0);
        } else {
            objects.sort_by(|a, b| {
                let x = a.bounding_box(time0, time1).unwrap().minimun.get(axis);
                let y = b.bounding_box(time0, time1).unwrap().maximum.get(axis);
                x.partial_cmp(&y).unwrap()
            });
            let mid = span / 2;
            let (object0, object1) = objects.split_at_mut(mid as usize);
            left = Arc::new(BvhNode::new(&mut object0.to_vec(), mid, time0, time1));
            right = Arc::new(BvhNode::new(&mut object1.to_vec(), span - mid, time0, time1));
        }
        let box11 = left.bounding_box(time0, time1).unwrap();
        let box22 = right.bounding_box(time0, time1).unwrap();
        Self {
            left,
            right,
            box1: Aabb::surrounding_box(box11, box22),
        }
    }
}
// fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: i32) -> bool {
//     let box_a = Aabb::new(Vec3::zero(), Vec3::zero());
//     let box_b = Aabb::new(Vec3::zero(), Vec3::zero());
//     let Option::Some(box_a) = a.bounding_box(0.0, 0.0);
//     let Option::Some( box_b) = b.bounding_box(0.0, 0.0);
//     //may need to add
//     if axis == 0 {
//         return box_a.minimun.x < box_b.minimun.x;
//     } else if axis == 1 {
//         return box_a.minimun.y < box_b.minimun.y;
//     } else {
//         return box_a.minimun.z < box_b.minimun.z;
//     }
// }
//
// fn box_x_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
//     return box_compare(a, b, 0);
// }
//
// fn box_y_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
//     return box_compare(a, b, 1);
// }
//
// fn box_z_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
//     return box_compare(a, b, 2);
// }

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        if self.box1.hit(&r, t_min, t_max) { return None; }
        let _temp = self.left.hit(r, t_min, t_max);
        if let Option::Some(_temp1) = _temp {
            let hit_right = self.right.hit(r, t_min, _temp1.t);
            if let Option::Some(_temp2right) = hit_right {
                Some(_temp2right)
            } else {
                Some(_temp1)
            }
        } else {
            let hit_right = self.right.hit(r, t_min, t_max);
            if let Option::Some(_temp2right) = hit_right {
                Some(_temp2right)
            } else {
                None
            }
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        todo!()
    }
}