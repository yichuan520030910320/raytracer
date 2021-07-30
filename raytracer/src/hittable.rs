pub use crate::aabb::Aabb;
use crate::aarect::{XyRect, XzRect, YzRect};
use crate::material::Isotropic;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use rand::Rng;
use std::f64::consts::PI;

const INF: f64 = 1000000.0;

use crate::onb::Onb;
use std::sync::Arc;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
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

pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
    fn pdf_value(&self, _: &Vec3, _: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
} //相当于一个基类 在列表里面会去看是谁将它实例化（如圆等图形）

impl Hitrecord {
    pub fn grt_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let temptheta = (-p.z) / p.x;

        let mut phi = (temptheta).atan();
        phi += PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
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

#[allow(dead_code)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    #[allow(dead_code)]
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
        self.center0
            + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

#[allow(clippy::suspicious_operation_groupings)]
#[allow(clippy::needless_return)]
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
            rec.p = Ray::at(&r, rec.t);
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
        Some(output_box)
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
#[allow(clippy::suspicious_operation_groupings)]
impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let oc = r.ori - self.center;
        let a = Vec3::squared_length(&r.dic);
        let half_b = Vec3::dot(r.dic, oc);
        let c = Vec3::squared_length(&oc) - self.radius * self.radius;
        let discriminant = (half_b * half_b - a * c) as f64;
        if discriminant < 0.0 {
            None
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

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        let output = Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(output)
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        if self.hit(Ray::new(*o, *v, 0.0), 0.001, INF).is_some() {
            let costheta_max =
                (1.0 - (self.radius * self.radius) / (self.center - *o).squared_length()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - costheta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - *o;

        let distance_squared = direction.squared_length();
        let uvw = Onb::build_from(&direction);

        let temp = random_to_sphere(self.radius, distance_squared);

        uvw.local(temp.x, temp.y, temp.z)
    }
}

pub fn random_to_sphere(radius: f64, diatance_squared: f64) -> Vec3 {
    let r1 = random_doouble();
    let r2 = random_doouble();
    let z = 1.0 + r2 * ((1.0 - radius * radius / diatance_squared).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}

pub struct Box1 {
    pub(crate) box_min: Vec3,
    pub(crate) box_max: Vec3,
    pub(crate) sides: HittableList,
}

impl Hittable for Box1 {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Option::from(Aabb::new(self.box_min, self.box_max))
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
        Self {
            box_min: *p0,
            box_max: *p1,
            sides: world,
        }
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
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            Some(rec)
        } else {
            None
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
            Some(output_box)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
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
        let tempresult: bool;
        let mut bboxtemp = Aabb::new(Vec3::zero(), Vec3::zero());
        if p.bounding_box(0.0, 1.0).is_some() {
            tempresult = true;
        } else {
            tempresult = false;
        }

        let mut min1 = Vec3::new(INF, INF, INF);
        let mut max1 = Vec3::new(-INF, -INF, -INF);
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
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Option::from(self.bbox)
    }
}

#[allow(dead_code)]
pub struct RotateX {
    //相对观察视角物体旋转的角度
    pub(crate) ptr: Arc<dyn Hittable>,
    pub(crate) sin_theta: f64,
    pub(crate) cos_theta: f64,
    pub(crate) hasbox: bool,
    pub(crate) bbox: Aabb,
}

impl RotateX {
    #[allow(dead_code)]
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);

        let sinthetatemp = radians.sin();
        let costhetatemp = radians.cos();
        let tempresult: bool;
        let mut bboxtemp = Aabb::new(Vec3::zero(), Vec3::zero());
        if let Option::Some(_) = p.bounding_box(0.0, 1.0) {
            tempresult = true;
        } else {
            tempresult = false;
        }
        let mut min1 = Vec3::new(INF, INF, INF);
        let mut max1 = Vec3::new(-INF, -INF, -INF);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bboxtemp.maximum.x + (1.0 - i as f64) * bboxtemp.minimun.x;
                    let y = j as f64 * bboxtemp.maximum.y + (1.0 - j as f64) * bboxtemp.minimun.y;
                    let z = k as f64 * bboxtemp.maximum.z + (1.0 - k as f64) * bboxtemp.minimun.z;
                    let newy = costhetatemp * y + sinthetatemp * z;
                    let newz = -sinthetatemp * y + costhetatemp * z;
                    let tester = Vec3::new(x, newy, newz);
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

impl Hittable for RotateX {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let mut origin = r.ori;
        let mut direct = r.dic;
        origin.y = self.cos_theta * r.ori.y - self.sin_theta * r.ori.z;
        origin.z = self.sin_theta * r.ori.y + self.cos_theta * r.ori.z;
        direct.y = self.cos_theta * r.dic.y - self.sin_theta * r.dic.z;
        direct.z = self.sin_theta * r.dic.y + self.cos_theta * r.dic.z;
        let rotated_ray = Ray::new(origin, direct, r.tm);
        //let rec=Hitrecord::new(Vec3::zero(),Vec3::zero(),0.0,false,)

        if let Option::Some(mut rec) = self.ptr.hit(rotated_ray, t_min, t_max) {
            let mut p = rec.p;
            let mut nomal = rec.normal;
            p.y = self.cos_theta * rec.p.y + self.sin_theta * rec.p.z;
            p.z = -self.sin_theta * rec.p.y + self.cos_theta * rec.p.z;
            nomal.y = self.cos_theta * rec.normal.y + self.sin_theta * rec.normal.z;
            nomal.z = -self.sin_theta * rec.normal.y + self.cos_theta * rec.normal.z;
            rec.p = p;
            rec.set_face_normal(&rotated_ray, nomal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Option::from(self.bbox)
    }
}

#[allow(dead_code)]
pub struct RotateZ {
    //相对观察视角物体旋转的角度
    pub(crate) ptr: Arc<dyn Hittable>,
    pub(crate) sin_theta: f64,
    pub(crate) cos_theta: f64,
    pub(crate) hasbox: bool,
    pub(crate) bbox: Aabb,
}

impl RotateZ {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);

        let sinthetatemp = radians.sin();
        let costhetatemp = radians.cos();
        let tempresult: bool;
        let mut bboxtemp = Aabb::new(Vec3::zero(), Vec3::zero());
        if let Option::Some(_) = p.bounding_box(0.0, 1.0) {
            tempresult = true;
        } else {
            tempresult = false;
        }
        let mut min1 = Vec3::new(INF, INF, INF);
        let mut max1 = Vec3::new(-INF, -INF, -INF);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bboxtemp.maximum.x + (1.0 - i as f64) * bboxtemp.minimun.x;
                    let y = j as f64 * bboxtemp.maximum.y + (1.0 - j as f64) * bboxtemp.minimun.y;
                    let z = k as f64 * bboxtemp.maximum.z + (1.0 - k as f64) * bboxtemp.minimun.z;
                    let newx = costhetatemp * x + sinthetatemp * y;
                    let newy = -sinthetatemp * x + costhetatemp * y;
                    let tester = Vec3::new(newx, newy, z);
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

impl Hittable for RotateZ {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let mut origin = r.ori;
        let mut direct = r.dic;
        origin.x = self.cos_theta * r.ori.x - self.sin_theta * r.ori.y;
        origin.y = self.sin_theta * r.ori.x + self.cos_theta * r.ori.y;
        direct.x = self.cos_theta * r.dic.x - self.sin_theta * r.dic.y;
        direct.y = self.sin_theta * r.dic.x + self.cos_theta * r.dic.y;
        let rotated_ray = Ray::new(origin, direct, r.tm);
        //let rec=Hitrecord::new(Vec3::zero(),Vec3::zero(),0.0,false,)

        if let Option::Some(mut rec) = self.ptr.hit(rotated_ray, t_min, t_max) {
            let mut p = rec.p;
            let mut nomal = rec.normal;
            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.y;
            p.y = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.y;
            nomal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.y;
            nomal.y = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.y;
            rec.p = p;
            rec.set_face_normal(&rotated_ray, nomal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Option::from(self.bbox)
    }
}

unsafe impl Send for HittableList {}

unsafe impl Sync for HittableList {}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
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
    #[allow(clippy::needless_return)]
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

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in self.objects.iter() {
            sum += weight * object.pdf_value(o, v);
        }

        sum
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        if self.objects.len() == 1 {
            self.objects[0].random(o)
        } else {
            let k = (rand::thread_rng().gen_range(0..int_size)) as usize;
            self.objects[k].random(o)
        }
    }
}

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}
#[allow(clippy::needless_return)]
impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        if let Option::Some(mut rec1) = self.boundary.hit(r, -INF, INF) {
            if let Option::Some(mut rec2) = self.boundary.hit(r.clone(), rec1.t + 0.0001, INF) {
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
                let distangce_inside_boundary = (rec2.t - rec1.t.clone()) * ray_length;
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
                recreturn.t = rec1.t + hit_distance / ray_length;
                recreturn.p = r.at(recreturn.t);
                recreturn.normal = Vec3::new(1.0, 0.0, 0.0);
                recreturn.front_face = true;
                recreturn.mat_ptr = self.phase_function.clone();
                Some(recreturn)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
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

        Self {
            left,
            right,
            box1: Aabb::surrounding_box(box11, box22),
        }
    }
}
#[allow(clippy::needless_return)]
impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        if !self.box1.hit(&r, t_min, t_max) {
            return None;
        }
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
                return None;
            }
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        let outout = self.box1;
        Some(outout)
    }
}
