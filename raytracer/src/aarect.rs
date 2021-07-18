use std::sync::Arc;
use crate::hittable::{Material, Hittable, Hitrecord};
use crate::{Ray, Vec3};
use crate::aabb::Aabb;

pub struct XyRect{
    pub(crate) mp:Arc<dyn Material>,
    pub(crate) x0:f64,
    pub(crate) x1:f64,
    pub(crate) y0:f64,
    pub(crate) y1:f64,
    pub(crate) k:f64
}
impl XyRect{
    pub fn new(_x0:f64,_x1:f64,_y0:f64,_y1:f64,_k:f64,mat:Arc<dyn Material>)->Self{
       Self{
           mp: mat,
           x0: _x0,
           x1: _x1,
           y0: _y0,
           y1: _y1,
           k: _k
       }
    }
}
impl Hittable for XyRect{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let t=(self.k-r.ori.z)/r.dic.z;
        if t<t_min||t>t_max {
            return None;
        }
        let x=r.ori.x+t*r.dic.x;
        let y=r.ori.y+t*r.dic.y;
        if x<self.x0||x>self.x1||y<self.y0||y>self.y1 {
            return None;
        }
        let mut rec =Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());

    rec.u=(x-self.x0)/(self.x1-self.x0);
        rec.v=(y-self.y0)/(self.y1-self.y0);
        rec.t=t;
        let ourward_normal=Vec3::new(0.0,0.0,1.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr=self.mp.clone();
        rec.p=r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        Some(Aabb::new(Vec3::new(self.x0,self.y0,self.k-0.0001),Vec3::new(self.x1,self.y1,self.k+0.0001))
        ) }
}

pub struct XzRect{
    pub(crate) mp:Arc<dyn Material>,
    pub(crate) x0:f64,
    pub(crate) x1:f64,
    pub(crate) z0:f64,
    pub(crate) z1:f64,
    pub(crate) k:f64
}
impl XzRect{
    pub fn new(_x0:f64,_x1:f64,_z0:f64,_z1:f64,_k:f64,mat:Arc<dyn Material>)->Self{
        Self{
            mp: mat,
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1:_z1,
            k: _k,

        }
    }
}
impl Hittable for XzRect{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let t=(self.k-r.ori.y)/r.dic.y;
        if t<t_min||t>t_max {
            return None;
        }
        let x=r.ori.x+t*r.dic.x;
        let z=r.ori.z+t*r.dic.z;
        if x<self.x0||x>self.x1||z<self.z0||z>self.z1 {
            return None;
        }
        let mut rec =Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());

        rec.u=(x-self.x0)/(self.x1-self.x0);
        rec.v=(z-self.z0)/(self.z1-self.z0);
        rec.t=t;
        let ourward_normal=Vec3::new(0.0,1.0,0.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr=self.mp.clone();
        rec.p=r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        Some(Aabb::new(Vec3::new(self.x0,self.k-0.0001,self.z0),Vec3::new(self.x1,self.k+0.0001,self.z1))
        ) }
}


pub struct YzRect{
    pub(crate) mp:Arc<dyn Material>,
    pub(crate) y0:f64,
    pub(crate) y1:f64,
    pub(crate) z0:f64,
    pub(crate) z1:f64,
    pub(crate) k:f64
}
impl YzRect{
    pub fn new(_y0:f64,_y1:f64,_z0:f64,_z1:f64,_k:f64,mat:Arc<dyn Material>)->Self{
        Self{
            mp: mat,
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1:_z1,
            k: _k,
        }
    }
}
impl Hittable for YzRect{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let t=(self.k-r.ori.x)/r.dic.x;
        if t<t_min||t>t_max {
            return None;
        }
        let y=r.ori.y+t*r.dic.y;
        let z=r.ori.z+t*r.dic.z;
        if y<self.y0||y>self.y1||z<self.z0||z>self.z1 {
            return None;
        }
        let mut rec =Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());

        rec.u=(y-self.y0)/(self.y1-self.y0);
        rec.v=(z-self.z0)/(self.z1-self.z0);
        rec.t=t;
        let ourward_normal=Vec3::new(1.0,0.0,0.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr=self.mp.clone();
        rec.p=r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        Some(Aabb::new(Vec3::new(self.k-0.0001,self.y0,self.z0),Vec3::new(self.k+0.0001,self.y1,self.z1))
        ) }
}