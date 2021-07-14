pub use crate::vec3::Vec3;
pub use crate::ray::Ray;
use std::f64::consts::PI;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}


fn degree_to_radians(degrees:f64)->f64{
    return degrees*PI/180.0;
}
impl Camera {
    pub fn new(vfov:f64,aspect_ratio:f64) -> Self {
        let theta=degree_to_radians(vfov);
        let h=(theta/2.0).tan();
        let view_heigth=2.0*h;
        let view_width=aspect_ratio*view_heigth;

        // let ratio: f64 = 16.0 / 9.0;
        // let view_heigth: f64 = 2.0;
        // let view_width = (view_heigth * ratio) as f64;
        let focallength = 1.0;
        Self {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },

            horizontal: Vec3 {
                x: view_width as f64,
                y: 0.0,
                z: 0.0,
            },

            vertical: Vec3 {
                x: 0.0,
                y: view_heigth,
                z: 0.0,
            },
            lower_left_corner: Vec3 {
                x:  - view_width as f64/2.0 ,
                y:  - view_heigth/2.0,
                z: - focallength,

            },
        }
    }
    pub fn get_ray(&self,u:f64,v:f64)->Ray{
        Ray::new(
          self.origin,self.lower_left_corner+self.horizontal*u+self.vertical*v-self.origin
        )
    }
}