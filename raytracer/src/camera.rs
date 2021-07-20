pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use crate::{ range_random_double};
use std::f64::consts::PI;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

fn degree_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        _time0: f64,
        _time1: f64,
    ) -> Self {
        let theta = degree_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let view_heigth = 2.0 * h;
        let view_width = aspect_ratio * view_heigth;
        let w = Vec3::unit(lookfrom - lookat);
        let u = Vec3::unit(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let horizontemp = u * view_width * focus_dist;
        let verticaltemp = v * view_heigth * focus_dist;

        // let ratio: f64 = 16.0 / 9.0;
        // let view_heigth: f64 = 2.0;
        // let view_width = (view_heigth * ratio) as f64;
        Self {
            origin: Vec3 {
                x: lookfrom.x,
                y: lookfrom.y,
                z: lookfrom.z,
            },

            horizontal: Vec3 {
                x: horizontemp.x,
                y: horizontemp.y,
                z: horizontemp.z,
            },

            vertical: Vec3 {
                x: verticaltemp.x,
                y: verticaltemp.y,
                z: verticaltemp.z,
            },
            lower_left_corner: Vec3 {
                x: lookfrom.x - horizontemp.x / 2.0 - verticaltemp.x / 2.0 - w.x * focus_dist,
                y: lookfrom.y - horizontemp.y / 2.0 - verticaltemp.y / 2.0 - w.y * focus_dist,
                z: lookfrom.z - horizontemp.z / 2.0 - verticaltemp.z / 2.0 - w.z * focus_dist,
            },
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0: _time0,
            time1: _time1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x.clone() + self.v * rd.y.clone();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            range_random_double(self.time0, self.time1),
        )
    }
}
