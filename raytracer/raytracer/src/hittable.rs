pub use crate::aabb::Aabb;
use crate::aarect::{XyRect, XzRect, YzRect};
use crate::earth;
use crate::material::Isotropic;
pub use crate::material::Material;
pub use crate::ray::Ray;
use crate::texture::Texture;
pub use crate::vec3::Vec3;
use rand::Rng;
use std::f64::consts::PI;
use std::f64::INFINITY;
use std::sync::Arc;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn fmin1(a: f64, b: f64) -> f64 {
    if a < b {
        return a;
    }
    return b;
}

fn fmax1(a: f64, b: f64) -> f64 {
    if a < b {
        return b;
    }
    return a;
}

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
} //相当于一个基类 在列表里面会去看是谁将它实例化（如圆等图形）

impl Hitrecord {
    pub fn grt_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let temptheta = (-p.z) / p.x;

        let mut phi = (temptheta).atan();
        phi = phi + PI;
        *u = *&mut (phi / (2.0 * PI));
        *v = *&mut (theta / PI);
    }
    pub fn new(
        p: Vec3,
        normal: Vec3,
        t: f64,
        front_face: bool,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        Self {
            p,
            normal,
            t,
            u: 0.0,
            v: 0.0,
            front_face,
            mat_ptr,
        }
    }

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
    pub fn new(
        cen0: Vec3,
        cen1: Vec3,
        _time0: f64,
        _time1: f64,
        r: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
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
        return self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0));
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
    pub fn new(
        p: Vec3,
        normal: Vec3,
        t: f64,
        center: Vec3,
        radius: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        Self {
            p,
            normal,
            t,
            center,
            radius,
            mat_ptr,
        }
    }
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
        let aa = time0;
        let bb = time1;
        //println!("sphere srrounding");
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
        let aa = time0;
        let bb = time1;
        return Option::from(Aabb::new(self.box_min, self.box_max));
    }
}

impl Box1 {
    pub fn new(p0: &Vec3, p1: &Vec3, ptr: Arc<dyn Material>) -> Self {
        let mut world = HittableList { objects: vec![] };
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
        world.add(Arc::new(obj1));
        world.add(Arc::new(obj2));
        world.add(Arc::new(obj3));
        world.add(Arc::new(obj4));
        world.add(Arc::new(obj5));
        world.add(Arc::new(obj6));
        return Self {
            box_min: *p0,
            box_max: *p1,
            sides: world,
        };
    }
}

pub struct Translate {
    pub(crate) ptr: Arc<dyn Hittable>,
    pub(crate) offset: Vec3,
}

impl Translate {
    pub fn new(ptr: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let moved_r = Ray::new(r.ori - self.offset, r.dic, r.tm);
        if let Option::Some(mut rec) = self.ptr.hit(moved_r, t_min, t_max) {
            rec.p = rec.p + self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            return Some(rec);
        } else {
            return None;
        }

        // Option::from(if let Option::Some(mut rec) = self.ptr.hit(moved_r, t_min, t_max) {
        //     rec.p += self.offset;
        //     rec.set_face_normal(&moved_r, rec.normal);
        //     rec
        // } else {
        //     None
        // })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if let Option::Some(mut output_box) = self.ptr.bounding_box(time0, time1) {
            output_box = Aabb::new(
                output_box.minimun + self.offset,
                output_box.maximum + self.offset,
            );
            return Some(output_box);
        } else {
            return None;
        }
    }
}

pub struct RotateY {
    //相对观察视角物体旋转的角度
    pub(crate) ptr: Arc<dyn Hittable>,
    pub(crate) sin_theta: f64,
    pub(crate) cos_theta: f64,
    pub(crate) hasbox: bool,
    pub(crate) bbox: Aabb,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);

        let sinthetatemp = radians.sin();
        let costhetatemp = radians.cos();
        let mut tempresult = false;
        let mut bboxtemp = Aabb::new(Vec3::zero(), Vec3::zero());
        if let Option::Some(bboxtemp) = p.bounding_box(0.0, 1.0) {
            tempresult = true;
        } else {
            tempresult = false;
        }
        let mut min1 = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max1 = Vec3::new(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bboxtemp.maximum.x + (1.0 - i as f64) * bboxtemp.minimun.x;
                    let y = j as f64 * bboxtemp.maximum.y + (1.0 - j as f64) * bboxtemp.minimun.y;
                    let z = k as f64 * bboxtemp.maximum.z + (1.0 - k as f64) * bboxtemp.minimun.z;
                    let newx = costhetatemp * x + sinthetatemp * z;
                    let newz = -sinthetatemp * x + costhetatemp * z;
                    let tester = Vec3::new(newx, y, newz);
                    min1.x = fmin1(min1.x, tester.x);
                    max1.x = fmax1(max1.x, tester.x);
                    min1.y = fmin1(min1.y, tester.y);
                    max1.y = fmax1(max1.y, tester.y);
                    min1.z = fmin1(min1.z, tester.z);
                    max1.z = fmax1(max1.z, tester.z);
                }
            }
        }
        bboxtemp = Aabb::new(min1, max1);

        Self {
            ptr: p,
            sin_theta: sinthetatemp,
            cos_theta: costhetatemp,
            hasbox: tempresult,
            bbox: bboxtemp,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let mut origin = r.ori;
        let mut direct = r.dic;
        origin.x = self.cos_theta * r.ori.x - self.sin_theta * r.ori.z;
        origin.z = self.sin_theta * r.ori.x + self.cos_theta * r.ori.z;
        direct.x = self.cos_theta * r.dic.x - self.sin_theta * r.dic.z;
        direct.z = self.sin_theta * r.dic.x + self.cos_theta * r.dic.z;
        let rotated_ray = Ray::new(origin, direct, r.tm);
        //let rec=Hitrecord::new(Vec3::zero(),Vec3::zero(),0.0,false,)

        if let Option::Some(mut rec) = self.ptr.hit(rotated_ray, t_min, t_max) {
            let mut p = rec.p;
            let mut nomal = rec.normal;
            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;
            nomal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            nomal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;
            rec.p = p;
            rec.set_face_normal(&rotated_ray, nomal);
            return Some(rec);
        } else {
            return None;
        }

        // Option::from(if let Option::Some(mut rec) = self.ptr.hit(rotated_ray, t_min, t_max) {
        //     let mut p = rec.p;
        //     let mut nomal = rec.normal;
        //     p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        //     p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;
        //     nomal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.p.z;
        //     nomal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;
        //     rec.p = p;
        //     rec.set_face_normal(&rotated_ray, nomal);
        //     rec
        // } else {
        //     None
        // })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let aa = time0;
        let bb = time1;
        return Option::from(self.bbox);
    }
}
unsafe impl Send for HittableList {}
unsafe impl Sync for HittableList {}
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
        if self.objects.is_empty() {
            return None;
        }
        let mut output = Aabb::new(Vec3::zero(), Vec3::zero());
        let mut first_box = true;
        for object in self.objects.iter() {
            if let Option::Some(tempbox) = object.bounding_box(time0, time1) {
                if first_box {
                    output = tempbox;
                } else {
                    output = Aabb::surrounding_box(output, tempbox);
                }
                first_box = false;
            } else {
                return None;
            }
        }
        Some(output)
    }
}

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        if let Option::Some(mut rec1) = self.boundary.hit(r.clone(), -INFINITY, INFINITY) {
            //todo
            if let Option::Some(mut rec2) =
                self.boundary
                    .hit(r.clone(), rec1.t.clone() + 0.0001, INFINITY)
            {
                if rec1.t < t_min {
                    rec1.t = t_min
                };
                if rec2.t > t_max {
                    rec2.t = t_max
                };

                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let ray_length = r.dic.length();
                let distangce_inside_boundary = (rec2.t.clone() - rec1.t.clone()) * ray_length;
                let hit_distance = self.neg_inv_density * random_doouble().ln();

                if hit_distance > distangce_inside_boundary {
                    return None;
                }
                let mut recreturn = Hitrecord::new(
                    Vec3::zero(),
                    Vec3::zero(),
                    0.0,
                    false,
                    self.phase_function.clone(),
                );
                recreturn.t = rec1.t.clone() + hit_distance / ray_length;
                recreturn.p = r.at(recreturn.t);
                recreturn.normal = Vec3::new(1.0, 0.0, 0.0);
                recreturn.front_face = true;
                recreturn.mat_ptr = self.phase_function.clone();
                return Some(recreturn);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        return self.boundary.bounding_box(time0, time1);
    }
}

impl ConstantMedium {
    pub fn new(b: Arc<dyn Hittable>, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            phase_function: Arc::new(Isotropic::new(c)),
            neg_inv_density: (-1.0 / d),
        }
    }
}

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub box1: Aabb,
}

impl BvhNode {
    pub fn new(src_objects: Vec<Arc<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        let span = src_objects.len();
        let mut objects = src_objects;
        let axis = rand::thread_rng().gen_range(0..3);
        //todo
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if span == 1 {
            left = objects.remove(0);
            right = left.clone();
        } else if span == 2 {
            objects.sort_by(|a, b| {
                let x = a.bounding_box(time0, time1).unwrap().minimun.get(axis);
                let y = b.bounding_box(time0, time1).unwrap().minimun.get(axis);
                x.partial_cmp(&y).unwrap()
            });
            right = objects.remove(1);
            left = objects.remove(0);
        } else {
            //println!("span is {}", span);
            objects.sort_by(|a, b| {
                let x = a.bounding_box(time0, time1).unwrap().minimun.get(axis);
                let y = b.bounding_box(time0, time1).unwrap().minimun.get(axis);
                x.partial_cmp(&y).unwrap()
            });
            let mid = span / 2;
            let (object0, object1) = objects.split_at_mut(mid as usize);
            left = Arc::new(BvhNode::new(object0.to_vec(), time0, time1));
            right = Arc::new(BvhNode::new(object1.to_vec(), time0, time1));
        }
        let box11 = left.bounding_box(time0, time1).unwrap();
        let box22 = right.bounding_box(time0, time1).unwrap();

        // println!("new bvh");
        // println!("{:?}",box11.maximum);
        // println!("{:?}",box11.minimun);
        // println!("{:?}",box22.maximum);
        // println!("{:?}",box22.minimun);
        // println!("new end");
        Self {
            left,
            right,
            box1: Aabb::surrounding_box(box11, box22),
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        // println!("bvh   hit1  {}",t_min);
        // println!("bvh   hit2  {}",t_max);
        if (!self.box1.hit(&r, t_min, t_max)) {
            //println!("bvh wrong 1");
            return None;
        }
        let _temp = self.left.hit(r, t_min, t_max);
        if let Option::Some(_temp1) = _temp {
            let hit_right = self.right.hit(r, t_min, _temp1.t);
            if let Option::Some(_temp2right) = hit_right {
                //println!("bvh ac 1");
                Some(_temp2right)
            } else {
                //println!("bvh ac 2");
                Some(_temp1)
            }
        } else {
            let hit_right = self.right.hit(r, t_min, t_max);
            if let Option::Some(_temp2right) = hit_right {
                //todo
                // println!("bvh ac 3");
                Some(_temp2right)
            } else {
                //println!("bvh wrong 2");
                None
            }
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let aa = time0;
        let bb = time1;
        let outout = self.box1.clone();
        return Some(outout);
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
