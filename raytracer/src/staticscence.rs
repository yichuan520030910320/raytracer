use crate::aarect::{StaticXyRect, StaticXzRect, StaticYzRect};
use crate::hittable::{StaticBox1, StaticHittableList, StaticMovingSphere, StaticRotateY, StaticSphere, StaticTranslate, StaticHittable, StaticBvhNode};
use crate::material::{
    StaticDielectric, StaticDiffuseLight, StaticFlipFace, StaticLambertian, StaticMetal,
};
use crate::perlin::NoiseTexture;
use crate::run::{random_doouble, range_random_double};
use crate::texture::{CheckerTexture, StaticBaseColor, StaticImageTexture};
use std::sync::Arc;
use crate::vec3::Vec3;


pub fn two_texture_static() -> StaticHittableList {
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
        mat_ptr: StaticLambertian::new1(checker),
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
    let checker = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
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
        mat_ptr: StaticDielectric::new(1.5),
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
        mat_ptr: StaticMetal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    };
    world.add(Arc::new(material3));
    world
}

pub(crate) fn static_earth() -> StaticHittableList {
    let mut world = StaticHittableList { objects: vec![] };

    let checker = StaticImageTexture::new("earthmap.jpg");
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
            y: 0.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: StaticLambertian::new1(checker), //todo
    };
    world.add(Arc::new(below));
    world
}

pub(crate) fn static_cornell_box() -> StaticHittableList {
    let mut world = StaticHittableList { objects: vec![] };

    let red = StaticYzRect {
        mp: StaticLambertian::<StaticBaseColor>::new(Vec3::new(0.65, 0.05, 0.05)),
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(red));

    let green = StaticYzRect {
        mp: StaticLambertian::<StaticBaseColor>::new(Vec3::new(0.12, 0.45, 0.15)),

        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(green));
    let white1 = StaticXzRect {
        mp: StaticLambertian::<StaticBaseColor>::new(Vec3::new(0.73, 0.73, 0.73)),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(white1));
    let white2 = StaticXzRect {
        mp: StaticLambertian::<StaticBaseColor>::new(Vec3::new(0.73, 0.73, 0.73)),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(white2));

    let white3 = StaticXyRect {
        mp: StaticLambertian::<StaticBaseColor>::new(Vec3::new(0.73, 0.73, 0.73)),

        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(white3));

    // whitebox4 = Box1::new( &Vec3::new(130.0, 0.0, 65.0), &Vec3::new(295.0, 165.0, 230.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    //
    // world.add(
    //     Arc::new(whitebox4)
    // );

    // let mut whitebox1: Arc<dyn Hittable> = Arc::new(Box1::new(
    //     &Vec3::new(0.0, 0.0, 0.0),
    //     &Vec3::new(165.0, 330.0, 165.0),
    //     Arc::new((Lambertian::new(Vec3::new(0.73, 0.73, 0.73)))),
    // ));
    // whitebox1 = Arc::new(RotateY::new(whitebox1, 15.0));
    // whitebox1 = Arc::new(Translate::new(whitebox1, Vec3::new(265.0, 0.0, 295.0)));
    // world.add(whitebox1);

    let whitebox1 = StaticBox1::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 330.0, 165.0),
        StaticMetal::new(Vec3::new(0.8, 0.85, 0.88), 0.0),
    );
    let whitebox2 = StaticRotateY::new(whitebox1, 15.0);
    let whitebox3 = StaticTranslate::new(whitebox2, Vec3::new(265.0, 0.0, 295.0));
    world.add(Arc::new(whitebox3));

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
            x: 190.0,
            y: 90.0,
            z: 190.0,
        },
        radius: 90.0,
        mat_ptr: StaticDielectric::new(1.5),
    };
    world.add(Arc::new(material1));

    // let mut whitebox2: Arc<dyn Hittable> = Arc::new(Box1::new(
    //     &Vec3::new(0.0, 0.0, 0.0),
    //     &Vec3::new(165.0, 165.0, 165.0),
    //     Arc::new((Lambertian::new(Vec3::new(0.73, 0.73, 0.73)))),
    // ));
    // whitebox2 = Arc::new(RotateY::new(whitebox2, -18.0));
    // whitebox2 = Arc::new(Translate::new(whitebox2, Vec3::new(130.0, 0.0, 65.0)));
    // world.add(whitebox2);

    // let whitebox5 = Box1::new( &Vec3::new(265.0, 0.0, 295.0), &Vec3::new(430.0, 330.0, 460.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    // world.add(
    //     Arc::new(whitebox5)
    // );

    let light1 = StaticXzRect {
        mp: StaticDiffuseLight::<StaticBaseColor>::new(Vec3::new(15.0, 15.0, 15.0)),
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
    };
    let light1_bonus = Arc::new(StaticFlipFace::new(light1));
    world.add(light1_bonus);
    world
}
use  raytracer_codegen::random_scene_static_bvh;
random_scene_static_bvh!{}
pub fn static_bvh_random_scence()->StaticHittableList{

    let mut world = StaticHittableList { objects: vec![] };
    let checker = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let m=Vec3::new(1.0,1.0,1.0);
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
    let obj:Arc<dyn StaticHittable>=add_bvh_static();
    world.add(obj);
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
        mat_ptr: StaticDielectric::new(1.5),
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
        mat_ptr: StaticMetal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    };
    world.add(Arc::new(material3));
    world
}
