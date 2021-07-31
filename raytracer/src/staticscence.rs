use crate::hittable::{Sphere, HittableList, StaticSphere, StaticHittableList, MovingSphere, StaticMovingSphere};
use crate::run::{Vec3, range_random_double, random_doouble};
use crate::material::{Lambertian, StaticLambertian, Metal, Dielectric, StaticMetal, StaticDielectric};
use std::sync::Arc;
use crate::perlin::NoiseTexture;
use crate::texture::{CheckerTexture, StaticBaseColor};

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


pub(crate) fn static_random_sence() -> StaticHittableList {
    let mut world = StaticHittableList { objects: vec![] };
    let checker = CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    );
    let ground = StaticSphere {
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
        mat_ptr: StaticLambertian::new1(checker),
    };
    world.add(Arc::new(ground));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_doouble();
            let center = Vec3::new(
                a as f64 + 0.9 * random_doouble(),
                0.2,
                b as f64 + 0.9 * random_doouble(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let tex=StaticBaseColor::new(albedo);
                    let center2 = center + Vec3::new(0.0, range_random_double(0.0, 0.5), 0.0);
                    let temp = StaticMovingSphere {
                        center0: center,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat_ptr: StaticLambertian::<StaticBaseColor>::new(albedo),
                    };
                    world.add(Arc::new(temp));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::randomrange(0.5, 1.0);
                    let fuzz = range_random_double(0.0, 0.5);
                    let temp = StaticSphere {
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
                            x: center.x,
                            y: center.y,
                            z: center.z,
                        },
                        radius: 0.2,
                        mat_ptr: StaticMetal::new(albedo, fuzz),
                    };
                    world.add(Arc::new(temp));
                } else {
                    let temp = StaticSphere {
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
                            x: center.x,
                            y: center.y,
                            z: center.z,
                        },
                        radius: 0.2,
                        mat_ptr: StaticDielectric::new(1.5),
                    };

                    world.add(Arc::new(temp));
                }
            }
        }
    }
    let material1 = StaticSphere {
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
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr:StaticDielectric::new(1.5),
    };
    world.add(Arc::new(material1));
    let material2 = StaticSphere {
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
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr: StaticLambertian::<StaticBaseColor>::new(Vec3::new(0.4, 0.2, 0.1)),
    };
    world.add(Arc::new(material2));

    let material3 = StaticSphere {
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
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        mat_ptr:StaticMetal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    };
    world.add(Arc::new(material3));
    world
}
