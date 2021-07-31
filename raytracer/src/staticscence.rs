use crate::hittable::{Sphere, HittableList, StaticSphere, StaticHittableList};
use crate::run::Vec3;
use crate::material::{Lambertian, StaticLambertian};
use std::sync::Arc;
use crate::perlin::NoiseTexture;

pub fn two_texture_static()->StaticHittableList{
    let mut world = StaticHittableList { objects: vec![] };

    let checker = NoiseTexture::new(4.0);
    let below = StaticSphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        mat_ptr:StaticLambertian::new1(checker),
    };
    world.add(Arc::new(below));

    let checker1 = NoiseTexture::new(4.0);
    let above = StaticSphere {
        p: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        normal: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        t: 0.0,
        center: Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: StaticLambertian::new1(checker1),
    };
    world.add(Arc::new(above));

    world

}