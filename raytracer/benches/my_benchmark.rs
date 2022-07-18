use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use crate::testbench::fibonacci;
// use mycrate::fibonacci;
// use testbench::fibonacci;


pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
fn fibonacci1(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

const INF: f64 = 1000000.0;
// mod aabb;
// mod aarect;
// mod example_macro;
// mod hittable;
// mod material;
// mod onb;
// mod pdf;
// mod perlin;
// mod ray;
// mod rtweekend;
// mod run;
// mod scence;
// mod staticscence;
// mod texture;
// mod vec3;
// include!("aarect.rs");
// include!("aabb.rs");
// include!("camera.rs");
// include!("hittable.rs");
// include!("onb.rs");
// include!("material.rs");
// include!("scence.rs");
// include!("staticscence.rs");
// include!("pdf.rs");
// include!("ray.rs");
// include!("texture.rs");
// include!("vec3.rs");


use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use std::f64::INFINITY;
//scence.rs



#[allow(dead_code)]
pub(crate) fn two_spheres() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    // let triangle=Triangel::new(Vec3 {
    //     x: 0.0,
    //     y: 0.0,
    //     z: 0.0
    // }, Vec3 {
    //     x: -20.0,
    //     y: -10.0,
    //     z: 0.0
    // }, Vec3 {
    //     x: 20.0,
    //     y: -10.0,
    //     z: 0.0
    // }, Arc::new(Lambertian::new(Vec3::new(0.3,0.8,0.9))));
    // world.add(Arc::new(triangle));

    // let rectangle=XzRect::new(-5.0, 5.0, -5.0, 5.0, 0.0, Arc::new(Lambertian::new(Vec3::new(0.3,0.8,0.9))));
    // world.add(Arc::new(rectangle));

    let below = Sphere {
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
            y: -10.0,
            z: 0.0,
        },
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)), //
    };
    world.add(Arc::new(below));
    let checker1 = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    let above = Sphere {
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
            y: 10.0,
            z: 0.0,
        },
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian::new1(checker1)), //
    };
    world.add(Arc::new(above));
    world
}


#[allow(dead_code)]
pub(crate) fn two_Dielectric_spheres() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let checker = Arc::new(NoiseTexture::new(4.0));
    let below = Sphere {
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
        mat_ptr: Arc::new(Lambertian::new1(checker)),
    };
    world.add(Arc::new(below));

    let checker1 = Arc::new(NoiseTexture::new(4.0));
    let above = Sphere {
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
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    };
    world.add(Arc::new(above));

    world
}


#[allow(dead_code)]
pub(crate) fn random_sence() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let ground = Sphere {
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
        mat_ptr: Arc::new(Lambertian::new1(checker)),
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
                    let temp = MovingSphere {
                        center0: center,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat_ptr: Arc::new(Lambertian::new(albedo)),
                    };
                    world.add(Arc::new(temp));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::randomrange(0.5, 1.0);
                    let fuzz = range_random_double(0.0, 0.5);
                    let temp = Sphere {
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
                        mat_ptr: Arc::new(Metal::new(albedo, fuzz)),
                    };
                    world.add(Arc::new(temp));
                } else {
                    let temp = Sphere {
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
                        mat_ptr: Arc::new(Dielectric::new(1.5)),
                    };

                    world.add(Arc::new(temp));
                }
            }
        }
    }
    let material1 = Sphere {
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
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    };
    world.add(Arc::new(material1));
    let material2 = Sphere {
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
        mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    };
    world.add(Arc::new(material2));

    let material3 = Sphere {
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
        mat_ptr: Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    };
    world.add(Arc::new(material3));
    world
}

#[allow(dead_code)]
pub(crate) fn two_berlin_spheres() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let checker = Arc::new(NoiseTexture::new(4.0));
    let below = Sphere {
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
        mat_ptr: Arc::new(Lambertian::new1(checker)),
    };
    world.add(Arc::new(below));

    let checker1 = Arc::new(NoiseTexture::new(4.0));
    let above = Sphere {
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
        mat_ptr: Arc::new(Lambertian::new1(checker1)),
    };
    world.add(Arc::new(above));

    world
}

#[allow(dead_code)]
pub(crate) fn earth() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let checker = Arc::new(ImageTexture::new("earthmap.jpg"));
    let below = Sphere {
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
        mat_ptr: Arc::new(Lambertian::new1(checker)), //todo
    };
    world.add(Arc::new(below));
    world
}

#[allow(dead_code)]
pub(crate) fn simple_light() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let checker = Arc::new(NoiseTexture::new(4.0));
    let below = Sphere {
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
        mat_ptr: Arc::new(Lambertian::new1(checker)), //todo
    };
    world.add(Arc::new(below));

    let checker1 = Arc::new(NoiseTexture::new(4.0));
    let above = Sphere {
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
        mat_ptr: Arc::new(Lambertian::new1(checker1)), //todo
    };
    world.add(Arc::new(above));
    let difflight1 = XyRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(4.0, 4.0, 4.0))),
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
    };
    world.add(Arc::new(difflight1));
    let light = Sphere {
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
            y: 6.0,
            z: 0.0,
        },
        radius: 2.0,
        mat_ptr: Arc::new(DiffuseLight::new(Vec3::new(4.0, 4.0, 4.0))),
    };
    world.add(Arc::new(light));

    //todo
    world
}

#[allow(dead_code)]
pub(crate) fn cornell_box() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let red = YzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05))),
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(red));

    let green = YzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15))),

        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(green));
    let white1 = XzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(white1));
    let white2 = XzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(white2));

    let white3 = XyRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

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

    let mut whitebox1: Arc<dyn Hittable> = Arc::new(Box1::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 330.0, 165.0),
        Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0)),
    ));
    whitebox1 = Arc::new(RotateY::new(whitebox1, 15.0));
    whitebox1 = Arc::new(Translate::new(whitebox1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(whitebox1);

    let material1 = Sphere {
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
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    };
    world.add(Arc::new(material1));




    // let cornell_box = tobj::load_obj(
    //     //buddle
    //     "spherehigh.obj",
    //     &tobj::LoadOptions {
    //         single_index: true,
    //         triangulate: true,
    //         ..Default::default()
    //     },
    // );
    // assert!(cornell_box.is_ok());
    // let rate = 10.0 * 1.9;
    // let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
    // for (_i, m) in models.iter().enumerate() {
    //     let mesh = &m.mesh;
    //     let mut boxes2 = HittableList { objects: vec![] };
    //     for v in 0..mesh.indices.len() / 3 {
    //         let x1 = mesh.indices[3 * v];
    //         let x2 = mesh.indices[3 * v + 1];
    //         let x3 = mesh.indices[3 * v + 2];
    //         let triange = Triangel::new(
    //             Vec3 {
    //                 x: rate * mesh.positions[(3 * x1) as usize] as f64,
    //                 y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
    //                 z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
    //             },
    //             Vec3 {
    //                 x: rate * mesh.positions[(3 * x2) as usize] as f64,
    //                 y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
    //                 z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
    //             },
    //             Vec3 {
    //                 x: rate * mesh.positions[(3 * x3) as usize] as f64,
    //                 y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
    //                 z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
    //             },
    //             Arc::new(Dielectric::new(1.5)),
    //         );
    //         boxes2.add(Arc::new(triange));
    //     }
    //     let allin = Translate::new(
    //         Arc::new(RotateY::new(
    //             Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
    //             200.0,
    //         )),
    //         Vec3::new(190.0, 90.0, 90.0),
    //     );
    //     world.add(Arc::new(allin));
    // }

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

    let light1 = XzRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0))),
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
    };
    let light1_bonus = Arc::new(FlipFace::new(Arc::new(light1)));
    world.add(light1_bonus);
    world
}

pub(crate) fn cornell_box_rabbit() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let red = YzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05))),
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(red));

    let green = YzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15))),

        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(green));
    let white1 = XzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(white1));
    let white2 = XzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(white2));

    let white3 = XyRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(white3));

    // let mut whitebox1: Arc<dyn Hittable> = Arc::new(Box1::new(
    //     &Vec3::new(0.0, 0.0, 0.0),
    //     &Vec3::new(165.0, 330.0, 165.0),
    //     Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0)),
    // ));
    // whitebox1 = Arc::new(RotateY::new(whitebox1, 15.0));
    // whitebox1 = Arc::new(Translate::new(whitebox1, Vec3::new(265.0, 0.0, 295.0)));
    // world.add(whitebox1);

    let cornell_box = tobj::load_obj(
        //buddle
        "Buddha.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );
    assert!(cornell_box.is_ok());
    let rate = 10.0 * 10.0 * 1.9;
    let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        let mut boxes2 = HittableList { objects: vec![] };
        for v in 0..mesh.indices.len() / 3 {
            let x1 = mesh.indices[3 * v];
            let x2 = mesh.indices[3 * v + 1];
            let x3 = mesh.indices[3 * v + 2];
            let triange = Triangel::new(
                Vec3 {
                    x: rate * mesh.positions[(3 * x1) as usize] as f64,
                    y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x2) as usize] as f64,
                    y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x3) as usize] as f64,
                    y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                },
                Arc::new(Metal::new(Vec3::new(0.99, 0.78, 0.0), 0.1)),
            );
            boxes2.add(Arc::new(triange));
        }
        let allin = Translate::new(
            Arc::new(RotateY::new(
                Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                200.0,
            )),
            Vec3::new(425.0, 200.0, 345.0),
        );
        world.add(Arc::new(allin));
    }

    let cornell_box = tobj::load_obj(
        //rabbit
        "bunny.fine.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );
    assert!(cornell_box.is_ok());
    let rate = 100.0 * 10.0;
    let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        let mut boxes2 = HittableList { objects: vec![] };
        for v in 0..mesh.indices.len() / 3 {
            let x1 = mesh.indices[3 * v];
            let x2 = mesh.indices[3 * v + 1];
            let x3 = mesh.indices[3 * v + 2];
            let triange = Triangel::new(
                Vec3 {
                    x: rate * mesh.positions[(3 * x1) as usize] as f64,
                    y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x2) as usize] as f64,
                    y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x3) as usize] as f64,
                    y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                },
                Arc::new(Lambertian::new(Vec3::new(0.74218, 0.74218, 0.74218))),
            );
            boxes2.add(Arc::new(triange));
        }
        let allin = Translate::new(
            Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
            Vec3::new(190.0, -27.0, 190.0),
        );
        world.add(Arc::new(allin));
    }

    let cornell_box = tobj::load_obj(
        "patrick.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );
    let filejpg = "Char_Patrick.png";
    assert!(cornell_box.is_ok());
    let rate = 120.0;
    let (models, _materials) = cornell_box.expect("Failed to load OBJ file");

    // Materials might report a separate loading error if the MTL file wasn't found.
    // If you don't need the materials, you can generate a default here and use that
    // instead.

    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        let mut boxes2 = HittableList { objects: vec![] };
        for v in 0..mesh.indices.len() / 3 {
            let x1 = mesh.indices[3 * v];
            let x2 = mesh.indices[3 * v + 1];
            let x3 = mesh.indices[3 * v + 2];
            // u v is not the correct result //the patrick triangle may be uncorrect while look at the result pic//inmite the RUST org
            //todo
            let u1 = mesh.texcoords[(x1 * 2) as usize];
            let v1 = mesh.texcoords[(x1 * 2 + 1) as usize];
            let mat1 = ObjTexture::new(filejpg, u1 as f64, v1 as f64);
            let u2 = mesh.texcoords[(x2 * 2) as usize];
            let v2 = mesh.texcoords[(x2 * 2 + 1) as usize];
            let _mat2 = ObjTexture::new(filejpg, u2 as f64, v2 as f64);
            let u3 = mesh.texcoords[(x3 * 2) as usize];
            let v3 = mesh.texcoords[(x3 * 2 + 1) as usize];
            let _mat3 = ObjTexture::new(filejpg, u3 as f64, v3 as f64);
            //try to merge the three texture

            let triange = Triangel::new(
                Vec3 {
                    x: rate * mesh.positions[(3 * x1) as usize] as f64,
                    y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x2) as usize] as f64,
                    y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x3) as usize] as f64,
                    y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                },
                Arc::new(Lambertian::new1(Arc::new(mat1))),
                //Arc::new(Lambertian::new(Vec3::new(0.99,0.756,0.756)))
            );
            boxes2.add(Arc::new(triange));
        }
        let allin = Translate::new(
            Arc::new(RotateY::new(
                Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                160.0,
            )),
            Vec3::new(330.0, 0.0, 100.0),
        );

        world.add(Arc::new(allin));
    }
    //todo texture fail but can't know the result

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

    let light1 = XzRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(13.0, 13.0, 13.0))),
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
    };
    let light1_bonus = Arc::new(FlipFace::new(Arc::new(light1)));
    world.add(light1_bonus);
    world
}

#[allow(dead_code)]
pub(crate) fn cornell_smoke() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let red = YzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05))),
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(red));

    let green = YzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15))),

        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(green));
    let white1 = XzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
    };
    world.add(Arc::new(white1));
    let white2 = XzRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
    };
    world.add(Arc::new(white2));
    let white3 = XyRect {
        mp: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),

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
    let mut whitebox1: Arc<dyn Hittable> = Arc::new(Box1::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 330.0, 165.0),
        Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),
    ));
    whitebox1 = Arc::new(RotateY::new(whitebox1, 15.0));
    whitebox1 = Arc::new(Translate::new(whitebox1, Vec3::new(265.0, 0.0, 295.0)));
    whitebox1 = Arc::new(ConstantMedium::new(
        whitebox1,
        0.01,
        Vec3::new(0.0, 0.0, 0.0),
    ));
    world.add(whitebox1);
    let mut whitebox2: Arc<dyn Hittable> = Arc::new(Box1::new(
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(165.0, 165.0, 165.0),
        Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),
    ));
    whitebox2 = Arc::new(RotateY::new(whitebox2, -18.0));
    whitebox2 = Arc::new(Translate::new(whitebox2, Vec3::new(130.0, 0.0, 65.0)));
    whitebox2 = Arc::new(ConstantMedium::new(
        whitebox2,
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
    ));
    world.add(whitebox2);
    // let whitebox5 = Box1::new( &Vec3::new(265.0, 0.0, 295.0), &Vec3::new(430.0, 330.0, 460.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    // world.add(
    //     Arc::new(whitebox5)
    // );
    let light1 = XzRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(7.0, 7.0, 7.0))),
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,
    };
    world.add(Arc::new(light1));
    world
}

#[allow(dead_code)]
pub(crate) fn final_book2_scence() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let light1 = XzRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(7.0, 7.0, 7.0))),
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,
    };
    world.add(Arc::new(light1));

    let mut boxes1 = HittableList { objects: vec![] };
    let boxes_per_sides = 20;
    for i in 0..boxes_per_sides {
        for j in 0..boxes_per_sides {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w as f64;
            let z0 = -1000.0 + j as f64 * w as f64;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = range_random_double(1.0, 101.0);
            let z1 = z0 + w;
            let whitebox4 = Box1::new(
                &Vec3::new(x0, y0, z0),
                &Vec3::new(x1, y1, z1),
                Arc::new(Lambertian::new(Vec3::new(0.48, 0.83, 0.53))),
            );
            boxes1.add(Arc::new(whitebox4));
        }
    }
    world.add(Arc::new(BvhNode::new(boxes1.objects, 0.0, 1.0)));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let temp = MovingSphere {
        center0: center1,
        center1: center2,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.1))),
    };
    world.add(Arc::new(temp));

    let tempmetal = Sphere {
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
            y: 150.0,
            z: 145.0,
        },
        radius: 50.0,
        mat_ptr: Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
    };
    world.add(Arc::new(tempmetal));

    let tempdielectric = Sphere {
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
            x: 260.0,
            y: 150.0,
            z: 45.0,
        },
        radius: 50.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    };

    world.add(Arc::new(tempdielectric));

    let tempdielectric2 = Sphere {
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
            x: 360.0,
            y: 150.0,
            z: 145.0,
        },
        radius: 70.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    };

    world.add(Arc::new(tempdielectric2));
    // let mut boudary: Arc<dyn Hittable> = Arc::new(Sphere::new(
    //     Vec3::zero(),
    //     Vec3::zero(),
    //     0.0,
    //     Vec3::new(360.0, 150.0, 145.0),
    //     70.0,
    //     Arc::new(Dielectric::new(1.5)),
    // ));
    // boudary = Arc::new(ConstantMedium::new(boudary, 0.2, Vec3::new(0.2, 0.4, 0.9)));
    //world.add(boudary);
    // let mut boudary2: Arc<dyn Hittable> = Arc::new(Sphere::new(
    //     Vec3::zero(),
    //     Vec3::zero(),
    //     0.0,
    //     Vec3::new(0.0, 0.0, 0.0),
    //     5000.0,
    //     Arc::new(Dielectric::new(1.5)),
    // ));
    // boudary2 = Arc::new(ConstantMedium::new(
    //     boudary2,
    //     0.0001,
    //     Vec3::new(1.0, 1.0, 1.0),
    // ));
    //world.add(boudary2);
    let checker = Arc::new(ImageTexture::new("earthmap.jpg"));
    let below = Sphere {
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
            x: 400.0,
            y: 200.0,
            z: 400.0,
        },
        radius: 100.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)),
    };
    world.add(Arc::new(below));
    let checker = Arc::new(NoiseTexture::new(0.1));
    let pertext = Sphere {
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
            x: 220.0,
            y: 280.0,
            z: 300.0,
        },
        radius: 80.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)),
    };
    world.add(Arc::new(pertext));
    let mut boxes2 = HittableList { objects: vec![] };
    for _ in 0..1000 {
        let ground = Sphere {
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
            center: Vec3::randomrange(0.0, 165.0),
            radius: 10.0,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))),
        };
        boxes2.add(Arc::new(ground));
    }

    let allin = Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    world.add(allin);

    world
}

#[allow(dead_code)]
pub(crate) fn my_scence_ball_world() -> HittableList {
    let cam = Vec3::new(13.0, 2.0, 0.0);
    let mut world = HittableList { objects: vec![] };
    let light1 = XyRect {
        mp: Arc::new(DiffuseLight::new(Vec3::new(0.0, 0.0, -1.0))),
        x0: -30.0,
        x1: 30.0,
        y0: 0.0,

        k: 20.0, //高度
        y1: 6.0,
    };
    world.add(Arc::new(light1));

    let floortexture = Arc::new(ImageTexture::new("wondersky.jpg"));
    let floor = Sphere::new(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        0.0,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        100.0,
        Arc::new(Lambertian::new1(floortexture)),
    );
    let terminalfloor = Arc::new(Translate::new(
        Arc::new(RotateZ::new(Arc::new(floor), 60.0)),
        Vec3::new(0.0, -100.0, 0.0),
    ));

    world.add(terminalfloor);
    let mut boxes2 = HittableList { objects: vec![] };
    for _ in 0..2500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.01, 0.05);
        a.z = range_random_double(-50.0, 50.0);
        a.x = range_random_double(-50.0, 150.0);
        let ground = Sphere {
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
            center: a,
            radius: 0.05,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.3125, 0.0, 0.50))),
        };
        boxes2.add(Arc::new(ground));
    }
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.00, 0.02);
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-10.0, 10.0);
        let ground = Sphere {
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
            center: a,
            radius: 0.05,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.3125, 0.0, 0.50))),
        };
        boxes2.add(Arc::new(ground));
    }
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.00, 0.02);
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-1.0, 12.0);
        let ground = Sphere {
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
            center: a,
            radius: 0.05,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.0, 0.99, 0.0))), //green
        };
        boxes2.add(Arc::new(ground));
    }
    for _ in 0..600 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.01, 0.04);
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(1.0, 13.0);
        let ground = Sphere {
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
            center: a,
            radius: 0.03,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.99, 0.00, 0.0))), //red
        };
        boxes2.add(Arc::new(ground));
    }

    for _ in 0..1000 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.03, 0.04);
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-10.0, 14.0);
        let ground = Sphere {
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
            center: a,
            radius: 0.03,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.99, 0.41100, 0.7058))), //pink
        };
        boxes2.add(Arc::new(ground));
    }

    let sky1 = Arc::new(ImageTexture::new("sky.png"));
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.03, 0.04) + 0.005;
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-1.0, 14.0);
        let sky_sphere = Sphere {
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
            center: a,
            radius: 0.04,
            mat_ptr: Arc::new(Lambertian::new1(sky1.clone())), //todo
        };
        boxes2.add(Arc::new(sky_sphere));
    }
    let redtexture = Arc::new(ImageTexture::new("red2.jpg"));
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.03, 0.04) + 2.305;
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-1.0, 14.0);
        let sky_sphere = Sphere {
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
            center: a,
            radius: 0.04,
            mat_ptr: Arc::new(Lambertian::new1(redtexture.clone())), //todo
        };
        boxes2.add(Arc::new(sky_sphere));
    }

    let universetexture = Arc::new(ImageTexture::new("universal.jpg"));
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.03, 0.04) + 1.305;
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-1.0, 10.0);
        let sky_sphere = Sphere {
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
            center: a,
            radius: 0.03,
            mat_ptr: Arc::new(Lambertian::new1(universetexture.clone())), //todo
        };
        boxes2.add(Arc::new(sky_sphere));
    }

    let universetexture = Arc::new(ImageTexture::new("universal2.jpg"));
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.03, 0.04) + 1.305 - 0.5;
        a.z = range_random_double(-10.0, 10.0);
        a.x = range_random_double(-1.0, 10.0);
        let sky_sphere = Sphere {
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
            center: a,
            radius: 0.03,
            mat_ptr: Arc::new(Lambertian::new1(universetexture.clone())),
        };
        boxes2.add(Arc::new(sky_sphere));
    }

    let allin = Arc::new(Translate::new(
        Arc::new(RotateZ::new(
            Arc::new(RotateY::new(
                Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                0.0,
            )),
            0.0,
        )),
        Vec3::new(0.0, 0.0, 0.0),
    ));
    world.add(allin);

    let mut boxes_red = HittableList { objects: vec![] };
    for _ in 0..500 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.0, 2.0);
        a.z = range_random_double(-5.0, 5.0);
        a.x = range_random_double(-10.0, 9.0);
        let ground = Sphere {
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
            center: a,
            radius: 0.055,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.99, 0.27, 0.0))),
        };
        boxes_red.add(Arc::new(ground));
    }
    let allinred = Arc::new(BvhNode::new(boxes_red.objects, 0.0, 1.0));
    world.add(allinred);

    let mut boxes_yellow = HittableList { objects: vec![] };
    for _ in 0..400 {
        let mut a = Vec3::zero();
        a.y = range_random_double(1.90, 2.50);
        a.z = range_random_double(-5.0, 5.0);
        a.x = range_random_double(-20.0, 6.0);
        if (a - cam).length() < 0.1 {
            continue;
        }
        let ground = Sphere {
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
            center: a,
            radius: 0.0592,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.99, 0.388, 0.27801))),
        };
        boxes_yellow.add(Arc::new(ground));
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_yellow.objects, 0.0, 1.0));
    world.add(allinyellow);

    let mut boxes_pink = HittableList { objects: vec![] };
    for _ in 0..400 {
        let mut a = Vec3::zero();
        a.y = range_random_double(3.90, 4.50) - 0.5;
        a.z = range_random_double(-9.0, 9.0);
        a.x = range_random_double(-20.0, 9.0);
        if (a - cam).length() < 0.1 {
            continue;
        }
        let ground = Sphere {
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
            center: a,
            radius: 0.0592,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.901, 0.541, 0.727))), //todo
        };
        boxes_pink.add(Arc::new(ground));
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_pink.objects, 0.0, 1.0));
    world.add(allinyellow);

    let mut boxes_white = HittableList { objects: vec![] };
    for _ in 0..400 {
        let mut a = Vec3::zero();
        a.y = range_random_double(1.70, 2.90);
        a.z = range_random_double(-5.0, 5.0);
        a.x = range_random_double(-20.0, 11.0);
        if (a - cam).length() < 0.1 {
            continue;
        }
        let ground = Sphere {
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
            center: a,
            radius: 0.0792,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.99, 0.99, 0.99))), //todo
        };
        boxes_white.add(Arc::new(ground));
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_white.objects, 0.0, 1.0));
    world.add(allinyellow);

    let mut boxes_greenblue = HittableList { objects: vec![] };
    for i in -12..12 {
        for j in -12..12 {
            let mut a = Vec3::zero();
            a.y = range_random_double(2.8, 4.90);
            a.z = i as f64 * 0.5;
            a.x = j as f64 * 0.5;
            if (a - cam).length() < 0.1 {
                continue;
            }
            let ground = Sphere {
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
                center: a,
                radius: 0.0792,
                mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.529, 0.8078, 0.9215))), //todo
            };
            boxes_greenblue.add(Arc::new(ground));
        }
    }
    for i in -11..11 {
        for j in -11..11 {
            let mut a = Vec3::zero();
            a.y = range_random_double(1.9, 2.20);
            a.z = i as f64 * 1.0 + 0.1 + j as f64 * 0.2;
            a.x = j as f64 * 1.0 + 0.1;
            if (a - cam).length() < 0.1 {
                continue;
            }
            let ground = Sphere {
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
                center: a,
                radius: 0.0792,
                mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.529, 0.8078, 0.9215))), //todo
            };
            boxes_greenblue.add(Arc::new(ground));
        }
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_greenblue.objects.clone(), 0.0, 1.0));
    world.add(allinyellow);

    let mut boxes_green = HittableList { objects: vec![] };
    for _ in 0..200 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.50, 1.160);
        a.z = range_random_double(-5.0, 5.0);
        a.x = range_random_double(-20.0, 12.0);
        if (a - cam).length() < 0.1 {
            continue;
        }
        let ground = Sphere {
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
            center: a,
            radius: 0.0792,
            mat_ptr: Arc::new(Metal::new(Vec3::new(0.564, 0.933, 0.564), 0.1)), //todo
        };
        boxes_green.add(Arc::new(ground));
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_green.objects, 0.0, 1.0));
    world.add(allinyellow);

    let mut boxes_dark_green = HittableList { objects: vec![] };
    for _ in 0..600 {
        let mut a = Vec3::zero();
        a.y = range_random_double(0.05, 0.15);
        a.z = range_random_double(-5.0, 5.0);
        a.x = range_random_double(-20.0, 12.0);
        if (a - cam).length() < 0.1 {
            continue;
        }
        let ground = Sphere {
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
            center: a,
            radius: 0.0392,
            mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.0, 0.3933, 0.0))), //todo
        };
        boxes_dark_green.add(Arc::new(ground));
    }
    let allinyellow = Arc::new(BvhNode::new(boxes_dark_green.objects, 0.0, 1.0));
    world.add(allinyellow);

    //
    let mut tennis_unit = HittableList { objects: vec![] };
    let tennis = Arc::new(ImageTexture::new("tennis.png"));
    for i in -8..8 {
        if i == 0 {
            continue;
        }
        let xx = i as f64 * (1.7 / 8.0);
        let sky_sphere = Sphere {
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
            radius: 0.1160,
            mat_ptr: Arc::new(Lambertian::new1(tennis.clone())), //todo
            center: Vec3 {
                x: 0.0,
                y: 3.0 - xx * xx,
                z: xx,
            },
        };
        tennis_unit.add(Arc::new(sky_sphere)); //
    }

    let allteniss = Translate::new(
        Arc::new(RotateY::new(Arc::new(tennis_unit), 60.0)),
        Vec3::new(6.0, -0.05, -1.0),
    );
    world.add(Arc::new(allteniss));

    let mut glass_unit = HittableList { objects: vec![] };
    for i in -8..8 {
        if i == 0 {
            continue;
        }
        let xx = i as f64 * (1.75 / 8.0);

        let sky_sphere = Sphere {
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
            radius: 0.0960,
            mat_ptr: Arc::new(Dielectric::new(1.5)),
            center: Vec3 {
                x: 0.0,
                y: 2.0 - 0.667 * xx * xx,
                z: xx,
            },
        };
        glass_unit.add(Arc::new(sky_sphere)); //
    }

    let allteniss = Translate::new(
        Arc::new(RotateY::new(Arc::new(glass_unit), -30.0)),
        Vec3::new(9.0, -0.05, 2.0),
    );
    world.add(Arc::new(allteniss));
    world
}

#[allow(dead_code)]
pub(crate) fn obj() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let cornell_box = tobj::load_obj(
        "bunny_1k.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );
    assert!(cornell_box.is_ok());
    let rate = 200.0;
    let (models, materials) = cornell_box.expect("Failed to load OBJ file");

    // Materials might report a separate loading error if the MTL file wasn't found.
    // If you don't need the materials, you can generate a default here and use that
    // instead.
    let materials = materials.expect("Failed to load MTL file");

    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("model[{}].incidices: {}", i, mesh.indices.len() / 3);

        assert!(mesh.indices.len() % 3 == 0);
        println!("len  is {}", mesh.indices.len());

        let mut boxes2 = HittableList { objects: vec![] };
        for v in 0..mesh.indices.len() / 3 {
            // println!(
            //     "  indices  v[{}] = ({}, {}, {})",
            //     v,
            //     mesh.indices[3 * v],
            //     mesh.indices[3 * v + 1],
            //     mesh.indices[3 * v + 2]
            // );
            let x1 = mesh.indices[3 * v];
            let x2 = mesh.indices[3 * v + 1];
            let x3 = mesh.indices[3 * v + 2];

            let triange = Triangel::new(
                Vec3 {
                    x: rate * mesh.positions[(3 * x1) as usize] as f64,
                    y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x2) as usize] as f64,
                    y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x3) as usize] as f64,
                    y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                },
                Arc::new(Lambertian::new(Vec3::new(0.99, 0.78, 0.0))),
            );

            // println!("triangle  vec  1is {:?}",triange.a1);
            // println!("triangle  vec 2 is {:?}",triange.a2);
            // println!("triangle  vec3  is {:?}",triange.a3);

            boxes2.add(Arc::new(triange));
        }
        let allin = BvhNode::new(boxes2.objects, 0.0, 1.0);
        world.add(Arc::new(allin));

        // println!("model[{}].name = \'{}\'", i, m.name);
        // println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);
        //
        // println!(
        //     "Size of model[{}].face_arities: {}",
        //     i,
        //     mesh.face_arities.len()
        // );

        let mut next_face = 0;
        for f in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
            println!("    face[{}] = {:?}", f, face_indices);
            next_face = end;
        }

        // Normals and texture coordinates are also loaded, but not printed in this example
        // println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        //
        // assert!(mesh.positions.len() % 3 == 0);
        // for v in 0..mesh.positions.len() / 3 {
        //     println!(
        //         "    v[{}] = ({}, {}, {})",
        //         v,
        //         mesh.positions[3 * v],
        //         mesh.positions[3 * v + 1],
        //         mesh.positions[3 * v + 2]
        //     );
        // }
    }

    for (i, m) in materials.iter().enumerate() {
        println!("material[{}].name = \'{}\'", i, m.name);
        println!(
            "    material.Ka = ({}, {}, {})",
            m.ambient[0], m.ambient[1], m.ambient[2]
        );
        println!(
            "    material.Kd = ({}, {}, {})",
            m.diffuse[0], m.diffuse[1], m.diffuse[2]
        );
        println!(
            "    material.Ks = ({}, {}, {})",
            m.specular[0], m.specular[1], m.specular[2]
        );
        println!("    material.Ns = {}", m.shininess);
        println!("    material.d = {}", m.dissolve);
        println!("    material.map_Ka = {}", m.ambient_texture);
        println!("    material.map_Kd = {}", m.diffuse_texture);
        println!("    material.map_Ks = {}", m.specular_texture);
        println!("    material.map_Ns = {}", m.shininess_texture);
        println!("    material.map_Bump = {}", m.normal_texture);
        println!("    material.map_d = {}", m.dissolve_texture);

        for (k, v) in &m.unknown_param {
            println!("    material.{} = {}", k, v);
        }
    }

    world
}

pub(crate) fn obj_with_texture() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let cornell_box = tobj::load_obj(
        "patrick.obj",
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
    );
    let filejpg = "Char_Patrick.png";
    assert!(cornell_box.is_ok());
    let rate = 10000.0;
    let (models, materials) = cornell_box.expect("Failed to load OBJ file");

    // Materials might report a separate loading error if the MTL file wasn't found.
    // If you don't need the materials, you can generate a default here and use that
    // instead.
    let materials = materials.expect("Failed to load MTL file");

    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("model[{}].incidices: {}", i, mesh.indices.len() / 3);
        println!(
            "   normal  incidieces len   is  {}",
            mesh.normal_indices.len()
        );
        println!(
            "  texture  incidieces len   is  {}",
            mesh.texcoord_indices.len()
        );

        println!("model[{}].incidices: {}", i, mesh.indices.len() / 3);

        assert!(mesh.indices.len() % 3 == 0);

        let mut boxes2 = HittableList { objects: vec![] };
        for v in 0..mesh.indices.len() / 3 {
            println!(
                "  indices  v[{}] = ({}, {}, {})",
                v,
                mesh.indices[3 * v],
                mesh.indices[3 * v + 1],
                mesh.indices[3 * v + 2]
            );
            let x1 = mesh.indices[3 * v];
            let x2 = mesh.indices[3 * v + 1];
            let x3 = mesh.indices[3 * v + 2];
            // u v is not the correct result //the patrick triangle may be uncorrect while look at the result pic//inmite the RUST org
            //todo
            let u1 = mesh.texcoords[(x1 * 2) as usize];
            let v1 = mesh.texcoords[(x1 * 2 + 1) as usize];
            let mat1 = ObjTexture::new(filejpg, u1 as f64, v1 as f64);
            let u2 = mesh.texcoords[(x2 * 2) as usize];
            let v2 = mesh.texcoords[(x2 * 2 + 1) as usize];
            let _mat2 = ObjTexture::new(filejpg, u2 as f64, v2 as f64);
            let u3 = mesh.texcoords[(x3 * 2) as usize];
            let v3 = mesh.texcoords[(x3 * 2 + 1) as usize];
            let _mat3 = ObjTexture::new(filejpg, u3 as f64, v3 as f64);
            //try to merge the three texture

            let triange = Triangel::new(
                Vec3 {
                    x: rate * mesh.positions[(3 * x1) as usize] as f64,
                    y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x2) as usize] as f64,
                    y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                },
                Vec3 {
                    x: rate * mesh.positions[(3 * x3) as usize] as f64,
                    y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                    z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                },
                Arc::new(Lambertian::new1(Arc::new(mat1))),
            );

            boxes2.add(Arc::new(triange));
        }
        let allin = BvhNode::new(boxes2.objects, 0.0, 1.0);
        world.add(Arc::new(allin));

        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);

        assert!(mesh.positions.len() % 3 == 0);
        for v in 0..mesh.positions.len() / 3 {
            println!(
                "    v[{}] = ({}, {}, {})",
                v,
                mesh.positions[3 * v],
                mesh.positions[3 * v + 1],
                mesh.positions[3 * v + 2]
            );
        }
    }

    for (i, m) in materials.iter().enumerate() {
        println!("material[{}].name = \'{}\'", i, m.name);
        println!(
            "    material.Ka = ({}, {}, {})",
            m.ambient[0], m.ambient[1], m.ambient[2]
        );
        println!(
            "    material.Kd = ({}, {}, {})",
            m.diffuse[0], m.diffuse[1], m.diffuse[2]
        );
        println!(
            "    material.Ks = ({}, {}, {})",
            m.specular[0], m.specular[1], m.specular[2]
        );
        println!("    material.Ns = {}", m.shininess);
        println!("    material.d = {}", m.dissolve);
        println!("    material.map_Ka = {}", m.ambient_texture);
        println!("    material.map_Kd = {}", m.diffuse_texture);
        println!("    material.map_Ks = {}", m.specular_texture);
        println!("    material.map_Ns = {}", m.shininess_texture);
        println!("    material.map_Bump = {}", m.normal_texture);
        println!("    material.map_d = {}", m.dissolve_texture);

        for (k, v) in &m.unknown_param {
            println!("    material.{} = {}", k, v);
        }
    }

    world
}

pub fn my_world() -> HittableList {
    let mut world = HittableList { objects: vec![] };

    let floor = XzRect {
        mp: Arc::new(Metal::new(Vec3::new(0.752, 0.752, 0.752), 0.850)),
        x0: -200.0,
        x1: 200.0,
        z0: -200.0,
        z1: 200.0,
        k: -10.0,
    };

    world.add(Arc::new(floor));

    let checker = Arc::new(ImageTexture::new("groud.jpg"));
    let backgroundmaterial = DiffuseLight::new1(checker);
    let mut backgroud_object = YzRect {
        mp: Arc::new(backgroundmaterial),
        y0: -180.0,
        y1: 180.0,
        z0: -10.0,
        z1: 110.0,
        k: -120.0,
    };
    let backgroud_object_rotate = RotateX::new(Arc::new(backgroud_object), 90.0);
    world.add(Arc::new(backgroud_object_rotate));

    let checker = Arc::new(ImageTexture::new("groud.jpg"));
    let backgroundmaterial = DiffuseLight::new1(checker);
    let mut backgroud_object = XyRect {
        mp: Arc::new(Metal::new(Vec3::new(0.752, 0.753, 0.752), 0.02)),
        //Arc::new(backgroundmaterial),
        x0: -80.0,
        x1: 90.0,
        y0: -10.0,
        y1: 110.0,

        k: -0.0,
    };
    let backgroud_translate = Translate::new(Arc::new(backgroud_object), Vec3::new(0.0, 0.0, 40.0));
    let backgroud_object_rotate = RotateY::new(Arc::new(backgroud_translate), 28.00);
    world.add(Arc::new(backgroud_object_rotate));

    let checker = Arc::new(ImageTexture::new("groud.jpg"));
    let backgroundmaterial = DiffuseLight::new1(checker);
    let mut backgroud_object_right = XyRect {
        mp: Arc::new(Metal::new(Vec3::new(0.752, 0.753, 0.752), 0.02)),
        x0: -80.0,
        x1: 90.0,
        y0: -10.0,
        y1: 110.0,

        k: -0.0,
    };
    let backgroud_translate =
        Translate::new(Arc::new(backgroud_object_right), Vec3::new(0.0, 0.0, -40.0));
    let backgroud_object_rotate = RotateY::new(Arc::new(backgroud_translate), -28.0);
    world.add(Arc::new(backgroud_object_rotate));

    // let cornell_box = tobj::load_obj(
    //     //buddle
    //     "Buddha.obj",
    //     &tobj::LoadOptions {
    //         single_index: true,
    //         triangulate: true,
    //         ..Default::default()
    //     },
    // );
    // assert!(cornell_box.is_ok());
    // let rate =  5.0 * 1.9;
    // let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
    // for (_i, m) in models.iter().enumerate() {
    //     let mesh = &m.mesh;
    //     let mut boxes2 = HittableList { objects: vec![] };
    //     for v in 0..mesh.indices.len() / 3 {
    //         let x1 = mesh.indices[3 * v];
    //         let x2 = mesh.indices[3 * v + 1];
    //         let x3 = mesh.indices[3 * v + 2];
    //         let triange = Triangel::new(
    //             Vec3 {
    //                 x: rate * mesh.positions[(3 * x1) as usize] as f64,
    //                 y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
    //                 z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
    //             },
    //             Vec3 {
    //                 x: rate * mesh.positions[(3 * x2) as usize] as f64,
    //                 y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
    //                 z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
    //             },
    //             Vec3 {
    //                 x: rate * mesh.positions[(3 * x3) as usize] as f64,
    //                 y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
    //                 z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
    //             },
    //             Arc::new(Metal::new(Vec3::new(0.99, 0.78, 0.0), 0.1)),
    //         );
    //         boxes2.add(Arc::new(triange));
    //     }
    //     let allin = Translate::new(
    //         Arc::new(RotateY::new(
    //             Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
    //             135.0,
    //         )),
    //         Vec3::new(10.0, -1.20, 30.0),
    //     );
    //     world.add(Arc::new(allin));
    // }

    //
    // let cornell_box = tobj::load_obj(
    //     //solider
    //     "FullBody_Decimated.OBJ",
    //     &tobj::LoadOptions {
    //         single_index: true,
    //         triangulate: true,
    //         ..Default::default()
    //     },
    // );
    // assert!(cornell_box.is_ok());
    // let rate = 0.051;
    // let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
    // for (_i, m) in models.iter().enumerate() {
    //     let mesh = &m.mesh;
    //     let mut boxes2 = HittableList { objects: vec![] };
    //     for v in 0..mesh.indices.len() / 3 {
    //         let x1 = mesh.indices[3 * v];
    //         let x2 = mesh.indices[3 * v + 1];
    //         let x3 = mesh.indices[3 * v + 2];
    //         let triange = Triangel::new(
    //             Vec3 {
    //                 x: rate * mesh.positions[(3 * x1) as usize] as f64,
    //                 y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
    //                 z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
    //             },
    //             Vec3 {
    //                 x: rate * mesh.positions[(3 * x2) as usize] as f64,
    //                 y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
    //                 z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
    //             },
    //             Vec3 {
    //                 x: rate * mesh.positions[(3 * x3) as usize] as f64,
    //                 y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
    //                 z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
    //             },
    //             Arc::new(Metal::new(Vec3::new(0.75218, 0.75218, 0.75218),0.1)),
    //         );
    //         boxes2.add(Arc::new(triange));
    //     }
    //     let allin = Translate::new(
    //         Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
    //         Vec3::new(-30.0, 20.0, -10.0),
    //     );
    //     let soilder=RotateX::new(Arc::new(allin),180.0);
    //     world.add(Arc::new(soilder));
    // }

    world
}

pub fn my_untimately() -> HittableList {
    {
        let mut world = HittableList { objects: vec![] };
        let checker = ImageTexture::new("medrian.jpg");
        let checker1 = ImageTexture::new("medrian3.jpg");
        let checker2 = ImageTexture::new("medrian2.jpg");
        let checker4 = ImageTexture::new("medrian2.jpg");
        let checker3 = ImageTexture::new("imagesfloor.jpg");

        let red = YzRect {
            mp: Arc::new(Lambertian::new1(Arc::new(checker1))),
            y0: -15.0,
            y1: 570.0,
            z0: 0.0,
            z1: 555.0,
            k: 0.0,
        };
        world.add(Arc::new(red));

        let green = YzRect {
            mp: Arc::new(Lambertian::new1(Arc::new(checker2))),

            y0: -15.0,
            y1: 570.0,
            z0: 0.0,
            z1: 555.0,
            k: 999.0,
        };
        world.add(Arc::new(green));
        let white1 = XzRect {
            mp: Arc::new(Lambertian::new1(Arc::new(checker3))),

            x0: 0.0,
            x1: 999.0,
            z0: 0.0,
            z1: 999.0,
            k: -15.0,
        };
        world.add(Arc::new(white1));
        let white2 = XzRect {
            mp: Arc::new(Lambertian::new1(Arc::new(checker))),

            x0: 0.0,
            x1: 999.0,
            z0: 0.0,
            z1: 999.0,
            k: 570.0,
        };
        world.add(Arc::new(white2));

        let white3 = XyRect {
            mp: Arc::new(Metal::new(Vec3::new(0.73, 0.73, 0.73), 0.1)),

            x0: 0.0,
            x1: 999.0,
            y0: -15.0,
            y1: 570.0,
            k: 555.0,
        };
        world.add(Arc::new(white3));

        let white3 = XyRect {
            mp: Arc::new(Lambertian::new1(Arc::new(checker4))),

            x0: -200.0 - 20.0,
            x1: 999.0 + 200.0 + 20.0,
            y0: -15.0 - 100.0 - 30.0,
            y1: 570.0 + 100.0 + 30.0,
            k: -800.0,
        };
        world.add(Arc::new(white3));

        //todo buddha
        let cornell_box = tobj::load_obj(
            //buddle
            "Buddha.obj",
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );
        let rate = 10.0 * 10.0 * 1.9;
        let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut boxes2 = HittableList { objects: vec![] };
            for v in 0..mesh.indices.len() / 3 {
                let x1 = mesh.indices[3 * v];
                let x2 = mesh.indices[3 * v + 1];
                let x3 = mesh.indices[3 * v + 2];
                let triange = Triangel::new(
                    Vec3 {
                        x: rate * mesh.positions[(3 * x1) as usize] as f64,
                        y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x2) as usize] as f64,
                        y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x3) as usize] as f64,
                        y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                    },
                    Arc::new(Metal::new(Vec3::new(0.99, 0.78, 0.0), 0.1)),
                );
                boxes2.add(Arc::new(triange));
            }
            let allin = Translate::new(
                Arc::new(RotateY::new(
                    Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                    200.0,
                )),
                Vec3::new(849.0, 200.0 - 15.0, 345.0),
            );
            world.add(Arc::new(allin));
        }

        //todo dragon

        let cornell_box = tobj::load_obj(
            //buddle
            "fixed.perfect.dragon.100K.0.07.obj",
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );
        let rate = 7.0 * 10.0 * 1.9;
        let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut boxes2 = HittableList { objects: vec![] };
            for v in 0..mesh.indices.len() / 3 {
                let x1 = mesh.indices[3 * v];
                let x2 = mesh.indices[3 * v + 1];
                let x3 = mesh.indices[3 * v + 2];
                let triange = Triangel::new(
                    Vec3 {
                        x: rate * mesh.positions[(3 * x1) as usize] as f64,
                        y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x2) as usize] as f64,
                        y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x3) as usize] as f64,
                        y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                    },
                    Arc::new(Metal::new(Vec3::new(0.0529, 0.2, 0.99), 0.1)),
                );
                boxes2.add(Arc::new(triange));
            }
            let allin = Translate::new(
                Arc::new(RotateY::new(
                    Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                    200.0,
                )),
                Vec3::new(649.0, 90.0, 375.0),
            );
            world.add(Arc::new(allin));
        }

        //todo sofa

        let cornell_box = tobj::load_obj(
            //rabbit
            "Koltuk.obj",
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );
        assert!(cornell_box.is_ok());
        let rate = 23.0 * 10.0;
        let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut boxes2 = HittableList { objects: vec![] };
            for v in 0..mesh.indices.len() / 3 {
                let x1 = mesh.indices[3 * v];
                let x2 = mesh.indices[3 * v + 1];
                let x3 = mesh.indices[3 * v + 2];
                let mut mat = Arc::new(Lambertian::new(Vec3::new(0.254, 0.411, 0.882)));

                if (rate * mesh.positions[(3 * x1 + 2) as usize] as f64) > 25.0 {
                    mat = Arc::new(Lambertian::new(Vec3::new(0.90174218, 0.7214218, 0.0)));
                }

                let triange = Triangel::new(
                    Vec3 {
                        x: rate * mesh.positions[(3 * x1) as usize] as f64,
                        y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x2) as usize] as f64,
                        y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x3) as usize] as f64,
                        y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                    },
                    mat,
                );
                boxes2.add(Arc::new(triange));
            }
            let allin = Translate::new(
                Arc::new(RotateY::new(
                    Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                    180.0,
                )),
                Vec3::new(245.0, 17.0, 480.0),
            );

            world.add(Arc::new(allin));
        }

        //todo rabbit
        let cornell_box = tobj::load_obj(
            //rabbit
            "bunny.fine.obj",
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );
        assert!(cornell_box.is_ok());
        let rate = 80.0 * 10.0;
        let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut boxes2 = HittableList { objects: vec![] };
            for v in 0..mesh.indices.len() / 3 {
                let x1 = mesh.indices[3 * v];
                let x2 = mesh.indices[3 * v + 1];
                let x3 = mesh.indices[3 * v + 2];
                let triange = Triangel::new(
                    Vec3 {
                        x: rate * mesh.positions[(3 * x1) as usize] as f64,
                        y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x2) as usize] as f64,
                        y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x3) as usize] as f64,
                        y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                    },
                    Arc::new(Lambertian::new(Vec3::new(0.74218, 0.74218, 0.74218))),
                );
                boxes2.add(Arc::new(triange));
            }
            let allin = Translate::new(
                Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                Vec3::new(240.0, 50.0, 350.0 + 110.0),
            );
            world.add(Arc::new(allin));
        }

        // // patrick
        //          let cornell_box = tobj::load_obj(
        //              "patrick.obj",
        //              &tobj::LoadOptions {
        //                  single_index: true,
        //                  triangulate: true,
        //                  ..Default::default()
        //              },
        //          );
        //          let filejpg = "Char_Patrick.png";
        //          assert!(cornell_box.is_ok());
        //          let rate = 120.0;
        //          let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        //          for (_i, m) in models.iter().enumerate() {
        //              let mesh = &m.mesh;
        //              let mut boxes2 = HittableList { objects: vec![] };
        //              for v in 0..mesh.indices.len() / 3 {
        //                  let x1 = mesh.indices[3 * v];
        //                  let x2 = mesh.indices[3 * v + 1];
        //                  let x3 = mesh.indices[3 * v + 2];
        //                  // u v is not the correct result //the patrick triangle may be uncorrect while look at the result pic//inmite the RUST org
        //                  //todo
        //                  let u1 = mesh.texcoords[(x1 * 2) as usize];
        //                  let v1 = mesh.texcoords[(x1 * 2 + 1) as usize];
        //                  let mat1 = ObjTexture::new(filejpg, u1 as f64, v1 as f64);
        //         let triange = Triangel::new(
        //                      Vec3 {
        //                          x: rate * mesh.positions[(3 * x1) as usize] as f64,
        //                          y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
        //                          z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
        //                      },
        //                      Vec3 {
        //                          x: rate * mesh.positions[(3 * x2) as usize] as f64,
        //                          y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
        //                          z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
        //                      },
        //                      Vec3 {
        //                          x: rate * mesh.positions[(3 * x3) as usize] as f64,
        //                          y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
        //                          z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
        //                      },
        //                      Arc::new(Lambertian::new1(Arc::new(mat1))),
        //                  );
        //                  boxes2.add(Arc::new(triange));
        //              }
        //              let allin = Translate::new(
        //                  Arc::new(RotateY::new(
        //                      Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
        //                      140.0,
        //                  )),
        //                  Vec3::new(370.0, 75.0, 350.0+110.0),
        //              );
        //              world.add(Arc::new(allin));
        //          }
        //
        // //
        // //bat
        //         let cornell_box = tobj::load_obj(
        //             "10485_Baseball_bat_v1_max2011_iteration-2.obj",
        //             &tobj::LoadOptions {
        //                 single_index: true,
        //                 triangulate: true,
        //                 ..Default::default()
        //             },
        //         );
        //         let filejpg = "10485_Baseball_bat_v1_diffuse.jpg";
        //         assert!(cornell_box.is_ok());
        //         let rate = 2.5;
        //         let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        //         for (_i, m) in models.iter().enumerate() {
        //             let mesh = &m.mesh;
        //             let mut boxes2 = HittableList { objects: vec![] };
        //             for v in 0..mesh.indices.len() / 3 {
        //                 let x1 = mesh.indices[3 * v];
        //                 let x2 = mesh.indices[3 * v + 1];
        //                 let x3 = mesh.indices[3 * v + 2];
        //                 // u v is not the correct result //the patrick triangle may be uncorrect while look at the result pic//inmite the RUST org
        //                 //todo
        //                 let u1 = mesh.texcoords[(x1 * 2) as usize];
        //                 let v1 = mesh.texcoords[(x1 * 2 + 1) as usize];
        //                 let mat1 = ObjTexture::new(filejpg, u1 as f64, v1 as f64);
        //                 let triange = Triangel::new(
        //                     Vec3 {
        //                         x: rate * mesh.positions[(3 * x1) as usize] as f64,
        //                         y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
        //                         z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
        //                     },
        //                     Vec3 {
        //                         x: rate * mesh.positions[(3 * x2) as usize] as f64,
        //                         y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
        //                         z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
        //                     },
        //                     Vec3 {
        //                         x: rate * mesh.positions[(3 * x3) as usize] as f64,
        //                         y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
        //                         z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
        //                     },
        //                     Arc::new(Lambertian::new1(Arc::new(mat1))),
        //                 );
        //                 boxes2.add(Arc::new(triange));
        //             }
        //             let allin = Translate::new(
        //                 Arc::new(RotateZ::new(Arc::new(RotateY::new(
        //                     Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
        //                     0.0,
        //                 )),-90.0)),
        //                 Vec3::new(445.0, 255.0, 360.0+110.0),
        //             );
        //             world.add(Arc::new(allin));
        //         }

        let cornell_box = tobj::load_obj(
            "patrick.obj",
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );
        let filejpg = "Char_Patrick.png";
        assert!(cornell_box.is_ok());
        let rate = 120.0;
        let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut boxes2 = HittableList { objects: vec![] };
            for v in 0..mesh.indices.len() / 3 {
                let x1 = mesh.indices[3 * v];
                let x2 = mesh.indices[3 * v + 1];
                let x3 = mesh.indices[3 * v + 2];
                // u v is not the correct result //the patrick triangle may be uncorrect while look at the result pic//inmite the RUST org
                //todo
                let u1 = mesh.texcoords[(x1 * 2) as usize];
                let v1 = mesh.texcoords[(x1 * 2 + 1) as usize];
                let mat1 = ObjTexture::new(filejpg, u1 as f64, v1 as f64);
                let triange = Triangel::new(
                    Vec3 {
                        x: rate * mesh.positions[(3 * x1) as usize] as f64,
                        y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x2) as usize] as f64,
                        y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x3) as usize] as f64,
                        y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                    },
                    Arc::new(Lambertian::new1(Arc::new(mat1))),
                );
                boxes2.add(Arc::new(triange));
            }
            let allin = Translate::new(
                Arc::new(RotateY::new(
                    Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                    140.0,
                )),
                Vec3::new(360.0, -15.0, 90.0),
            );
            world.add(Arc::new(allin));
        }

        //todo desk
        let cornell_box = tobj::load_obj(
            //rabbit
            "TABLE.obj",
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );
        assert!(cornell_box.is_ok());
        let rate = 0.440;
        let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut boxes2 = HittableList { objects: vec![] };
            for v in 0..mesh.indices.len() / 3 {
                let x1 = mesh.indices[3 * v];
                let x2 = mesh.indices[3 * v + 1];
                let x3 = mesh.indices[3 * v + 2];
                let triange = Triangel::new(
                    Vec3 {
                        x: rate * mesh.positions[(3 * x1) as usize] as f64,
                        y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x2) as usize] as f64,
                        y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x3) as usize] as f64,
                        y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                    },
                    Arc::new(Lambertian::new(Vec3::new(0.8, 0.33411, 0.000882))),
                );
                boxes2.add(Arc::new(triange));
            }
            let allin = Translate::new(
                Arc::new(RotateY::new(
                    Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                    90.0,
                )),
                Vec3::new(75.0, 88.0, 150.0),
            );

            world.add(Arc::new(allin));
        }

        let cornell_box = tobj::load_obj(
            //rabbit
            "Gaming_Chair.obj",
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );
        assert!(cornell_box.is_ok());
        let rate = 0.440 * 1.0 * 10.0;
        let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut boxes2 = HittableList { objects: vec![] };
            for v in 0..mesh.indices.len() / 3 {
                let x1 = mesh.indices[3 * v];
                let x2 = mesh.indices[3 * v + 1];
                let x3 = mesh.indices[3 * v + 2];
                let triange = Triangel::new(
                    Vec3 {
                        x: rate * mesh.positions[(3 * x1) as usize] as f64,
                        y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x2) as usize] as f64,
                        y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x3) as usize] as f64,
                        y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                    },
                    Arc::new(Dielectric::new(1.5)),
                    // Arc::new(Lambertian::new(Vec3::new(0.8, 0.33411, 0.000882))),
                );
                boxes2.add(Arc::new(triange));
            }
            let allin = Translate::new(
                Arc::new(RotateY::new(
                    Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                    -90.0,
                )),
                Vec3::new(195.0, -10.0, 150.0),
            );

            world.add(Arc::new(allin));
        }

        let cornell_box = tobj::load_obj(
            //rabbit
            "bigpc.obj",
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );
        assert!(cornell_box.is_ok());
        let rate = 0.0840 * 100.0 * 2.5;
        let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut boxes2 = HittableList { objects: vec![] };
            for v in 0..mesh.indices.len() / 3 {
                let x1 = mesh.indices[3 * v];
                let x2 = mesh.indices[3 * v + 1];
                let x3 = mesh.indices[3 * v + 2];
                let triange = Triangel::new(
                    Vec3 {
                        x: rate * mesh.positions[(3 * x1) as usize] as f64,
                        y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x2) as usize] as f64,
                        y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                    },
                    Vec3 {
                        x: rate * mesh.positions[(3 * x3) as usize] as f64,
                        y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                        z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                    },
                    Arc::new(Lambertian::new(Vec3::new(0.668, 0.6633411, 0.660882))),
                );
                boxes2.add(Arc::new(triange));
            }
            let allin = Translate::new(
                Arc::new(RotateY::new(
                    Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),
                    90.0,
                )),
                Vec3::new(85.0, 118.0, 100.0),
            );

            world.add(Arc::new(allin));
        }

        //todo lamp
        //
        // let cornell_box = tobj::load_obj(
        //     //lamp
        //     "lamp.obj",
        //     &tobj::LoadOptions {
        //         single_index: true,
        //         triangulate: true,
        //         ..Default::default()
        //     },
        // );
        // assert!(cornell_box.is_ok());
        // let rate = 80.0 * 10.0;
        // let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        // for (_i, m) in models.iter().enumerate() {
        //     let mesh = &m.mesh;
        //     let mut boxes2 = HittableList { objects: vec![] };
        //     for v in 0..mesh.indices.len() / 3 {
        //         let x1 = mesh.indices[3 * v];
        //         let x2 = mesh.indices[3 * v + 1];
        //         let x3 = mesh.indices[3 * v + 2];
        //         let triange = Triangel::new(
        //             Vec3 {
        //                 x: rate * mesh.positions[(3 * x1) as usize] as f64,
        //                 y: rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
        //                 z: rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
        //             },
        //             Vec3 {
        //                 x: rate * mesh.positions[(3 * x2) as usize] as f64,
        //                 y: rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
        //                 z: rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
        //             },
        //             Vec3 {
        //                 x: rate * mesh.positions[(3 * x3) as usize] as f64,
        //                 y: rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
        //                 z: rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
        //             },
        //             Arc::new(Lambertian::new(Vec3::new(0.99, 0.994218, 0.9974218))),
        //         );
        //         boxes2.add(Arc::new(triange));
        //     }
        //     let allin = Translate::new(
        //         Arc::new(RotateY::new(Arc::new(BvhNode::new(boxes2.objects, 0.0, 1.0)),90.0))
        //         ,
        //         Vec3::new(500.0, 555.0, 287.0),
        //     );
        //     world.add(Arc::new(allin));
        // }

        //
        // let light1=Sphere{
        //     p: Vec3::zero(),
        //     normal: Vec3::zero(),
        //     t: 0.0,
        //     center: Vec3 {
        //         x: 555.0,
        //         y: 278.0,
        //         z: 540.0
        //     },
        //     radius: 10.0,
        //     mat_ptr: Arc::new(DiffuseLight::new(Vec3::new(13.0, 13.0, 13.0))),
        // };
        //         let light1_bonus = Arc::new(FlipFace::new(Arc::new(light1)));
        //         world.add(light1_bonus);
        //
        //
        //         let light1=Sphere{
        //             p: Vec3::zero(),
        //             normal: Vec3::zero(),
        //             t: 0.0,
        //             center: Vec3 {
        //                 x: 520.0,
        //                 y: 278.0,
        //                 z: 520.0
        //             },
        //             radius: 10.0,
        //             mat_ptr: Arc::new(DiffuseLight::new(Vec3::new(13.0, 13.0, 13.0))),
        //         };
        //         let light1_bonus = Arc::new(FlipFace::new(Arc::new(light1)));
        //         world.add(light1_bonus);

        let light1 = XzRect {
            mp: Arc::new(DiffuseLight::new(Vec3::new(13.0, 13.0, 13.0))),
            x0: 213.0 + 222.0,
            x1: 343.0 + 222.0,
            z0: 227.0,
            z1: 332.0,
            k: 569.0,
        };
        let light1_bonus = Arc::new(FlipFace::new(Arc::new(light1)));
        world.add(light1_bonus);
        world
    }
}


//staticscence.rs


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

use raytracer_codegen::random_scene_static_bvh;
random_scene_static_bvh! {}
pub fn static_bvh_random_scence() -> StaticHittableList {
    let mut world = StaticHittableList { objects: vec![] };
    let checker = CheckerTexture::new(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let m = Vec3::new(1.0, 1.0, 1.0);
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
    let obj: Arc<dyn StaticHittable> = add_bvh_static();
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


//texture.rs
pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct CheckerTexture {
    color_value: Vec3,
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

#[derive(Clone)]
pub struct BaseColor {
    color: Vec3,
}

impl BaseColor {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for BaseColor {
    fn value(&self, _: f64, _: f64, _: &Vec3) -> Vec3 {
        //println!("{:?}",self.color);

        self.color
    }
}

impl CheckerTexture {
    pub fn new(_odd: Vec3, _even: Vec3) -> Self {
        Self {
            color_value: Vec3::zero(),
            odd: Arc::new(BaseColor::new(_odd)),
            even: Arc::new(BaseColor::new(_even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        }
    }
}

#[derive(Clone)]
pub struct ObjTexture {
    pub u: f64,
    pub v: f64,
    pub img: RgbImage,
}

#[derive(Clone)]
pub struct ImageTexture {
    pub width: i32,
    pub height: i32,
    pub bytes_per_scanline: i32,
    pub img: RgbImage,
}

impl ObjTexture {
    #[allow(dead_code)]
    pub fn new(filename: &str, u: f64, v: f64) -> Self {
        Self {
            u,
            v,
            img: image::open(filename).expect("failed").to_rgb8(),
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl Texture for ObjTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        let mut i = (self.u * ((self.img.width()) as f64)) as i32;
        let mut j = (self.v * ((self.img.height()) as f64)) as i32;
        if i >= self.img.width() as i32 {
            i = self.img.width() as i32 - 1;
        }
        if j >= self.img.height() as i32 {
            j = self.img.height() as i32 - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i as u32, j as u32);
        //let pixel=(self.data)+j*self.bytes_per_scanline+i*BYTES_PER_PIXEL;
        // println!(
        //     //"rnm  vec {:?}",
        //     Vec3::new(
        //         color_scale * (pixel[0] as f64),
        //         color_scale * (pixel[1] as f64),
        //         color_scale * (pixel[2] as f64),
        //     )
        // );

        Vec3::new(
            color_scale * (pixel[0] as f64),
            color_scale * (pixel[1] as f64),
            color_scale * (pixel[2] as f64),
        )
    }
}

impl ImageTexture {
    #[allow(dead_code)]
    pub fn new(filename: &str) -> Self {
        Self {
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
            img: image::open(filename).expect("failed").to_rgb8(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Vec3) -> Vec3 {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (u * ((self.img.width()) as f64)) as i32;
        let mut j = (v * ((self.img.height()) as f64)) as i32;
        if i >= self.img.width() as i32 {
            i = self.img.width() as i32 - 1;
        }
        if j >= self.img.height() as i32 {
            j = self.img.height() as i32 - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i as u32, j as u32);
        //let pixel=(self.data)+j*self.bytes_per_scanline+i*BYTES_PER_PIXEL;
        Vec3::new(
            color_scale * (pixel[0] as f64),
            color_scale * (pixel[1] as f64),
            color_scale * (pixel[2] as f64),
        )
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct StaticCheckerTexture<T1: Texture, T2: Texture> {
    color_value: Vec3,
    odd: T1,
    even: T2,
}

#[derive(Clone)]
pub struct StaticBaseColor {
    color: Vec3,
}

impl StaticBaseColor {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for StaticBaseColor {
    fn value(&self, _: f64, _: f64, _: &Vec3) -> Vec3 {
        self.color
    }
}

impl StaticCheckerTexture<StaticBaseColor, StaticBaseColor> {
    #[allow(dead_code)]
    pub fn new(_odd: Vec3, _even: Vec3) -> Self {
        Self {
            color_value: Vec3::zero(),
            odd: StaticBaseColor::new(_odd),
            even: StaticBaseColor::new(_even),
        }
    }
}

impl<T1: Texture, T2: Texture> Texture for StaticCheckerTexture<T1, T2> {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        }
    }
}

#[derive(Clone)]
pub struct StaticObjTexture {
    pub u: f64,
    pub v: f64,
    pub img: RgbImage,
}

#[derive(Clone)]
pub struct StaticImageTexture {
    pub width: i32,
    pub height: i32,
    pub bytes_per_scanline: i32,
    pub img: RgbImage,
}

impl StaticObjTexture {
    #[allow(dead_code)]
    pub fn new(filename: &str, u: f64, v: f64) -> Self {
        Self {
            u,
            v,
            img: image::open(filename).expect("failed").to_rgb8(),
        }
    }
}

#[allow(clippy::many_single_char_names)]
impl Texture for StaticObjTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        let mut i = (self.u * ((self.img.width()) as f64)) as i32;
        let mut j = (self.v * ((self.img.height()) as f64)) as i32;
        if i >= self.img.width() as i32 {
            i = self.img.width() as i32 - 1;
        }
        if j >= self.img.height() as i32 {
            j = self.img.height() as i32 - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i as u32, j as u32);
        //let pixel=(self.data)+j*self.bytes_per_scanline+i*BYTES_PER_PIXEL;
        // println!(
        //     "rnm  vec {:?}",
        //     Vec3::new(
        //         color_scale * (pixel[0] as f64),
        //         color_scale * (pixel[1] as f64),
        //         color_scale * (pixel[2] as f64),
        //     )
        // );

        Vec3::new(
            color_scale * (pixel[0] as f64),
            color_scale * (pixel[1] as f64),
            color_scale * (pixel[2] as f64),
        )
    }
}

impl StaticImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
            img: image::open(filename).expect("failed").to_rgb8(),
        }
    }
}

impl Texture for StaticImageTexture {
    fn value(&self, u: f64, v: f64, _: &Vec3) -> Vec3 {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (u * ((self.img.width()) as f64)) as i32;
        let mut j = (v * ((self.img.height()) as f64)) as i32;
        if i >= self.img.width() as i32 {
            i = self.img.width() as i32 - 1;
        }
        if j >= self.img.height() as i32 {
            j = self.img.height() as i32 - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i as u32, j as u32);
        //let pixel=(self.data)+j*self.bytes_per_scanline+i*BYTES_PER_PIXEL;
        Vec3::new(
            color_scale * (pixel[0] as f64),
            color_scale * (pixel[1] as f64),
            color_scale * (pixel[2] as f64),
        )
    }
}

//onb.rs


pub struct Onb {
    axis0: Vec3,
    axis1: Vec3,
    axis2: Vec3,
}

impl Onb {
    pub fn u(&self) -> Vec3 {
        self.axis0
    }
    pub fn v(&self) -> Vec3 {
        self.axis1
    }
    pub fn w(&self) -> Vec3 {
        self.axis2
    }
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u() * a + self.v() * b + self.w() * c
    }
    pub fn build_from(n: &Vec3) -> Self {
        let a;
        let axiss2 = n.unit();

        if axiss2.x.abs() > 0.9 {
            a = Vec3::new(0.0, 1.0, 0.0);
        } else {
            a = Vec3::new(1.0, 0.0, 0.0);
        }
        let axiss1 = Vec3::cross(axiss2, a).unit();
        let axiss0 = Vec3::cross(axiss2, axiss1);
        Self {
            axis0: axiss0,
            axis1: axiss1,
            axis2: axiss2,
        }
    }
}


//pdf.rs

pub trait Pdf: Send + Sync {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = Vec3::dot(direction.unit(), self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        let tempvec = Vec3::random_cosine_direction();
        self.uvw.local(tempvec.x, tempvec.y, tempvec.z)
    }
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        let ans = Onb::build_from(w);
        Self { uvw: ans }
    }
}

pub struct HittablePdf {
    o: Vec3,
    ptr: Arc<dyn Hittable>,
}

impl HittablePdf {
    #[allow(dead_code)]
    pub fn new(p: Arc<dyn Hittable>, orgin: &Vec3) -> Self {
        Self { o: *orgin, ptr: p }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

pub struct MixturePdf {
    p0: Arc<dyn Pdf>,
    p1: Arc<dyn Pdf>,
}

impl MixturePdf {
    #[allow(dead_code)]
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p0, p1 }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.p0.value(direction) * 0.5 + self.p1.value(direction) * 0.5
    }

    fn generate(&self) -> Vec3 {
        if random_doouble() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}

pub struct NoPdf {
    pub val: f64,
}

impl NoPdf {
    pub fn new() -> Self {
        Self { val: 0.0 }
    }
}

impl Pdf for NoPdf {
    fn value(&self, _: &Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::zero()
    }
}

//Static

pub struct StaticCosinePdf {
    uvw: Onb,
}

impl Pdf for StaticCosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = Vec3::dot(direction.unit(), self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        let tempvec = Vec3::random_cosine_direction();
        self.uvw.local(tempvec.x, tempvec.y, tempvec.z)
    }
}

impl StaticCosinePdf {
    #[allow(dead_code)]
    pub fn new(w: &Vec3) -> Self {
        let ans = Onb::build_from(w);
        Self { uvw: ans }
    }
}

pub struct StaticHittablePdf<'a, T: StaticHittable> {
    o: Vec3,
    ptr: &'a T,
}

impl<'a, T: StaticHittable> StaticHittablePdf<'a, T> {
    pub fn new(p: &'a T, orgin: &Vec3) -> Self {
        Self { o: *orgin, ptr: p }
    }
}

impl<'a, T: StaticHittable> Pdf for StaticHittablePdf<'a, T> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

pub struct StaticMixturePdf<'a, T1: Pdf, T2: Pdf> {
    p0: &'a T1,
    p1: &'a T2,
}

impl<'a, T1: Pdf, T2: Pdf> StaticMixturePdf<'a, T1, T2> {
    pub fn new(p0: &'a T1, p1: &'a T2) -> Self {
        Self { p0, p1 }
    }
}

impl<'a, T1: Pdf, T2: Pdf> Pdf for StaticMixturePdf<'a, T1, T2> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.p0.value(direction) * 0.5 + self.p1.value(direction) * 0.5
    }

    fn generate(&self) -> Vec3 {
        if random_doouble() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}

pub struct StaticNoPdf {
    pub val: f64,
}

impl StaticNoPdf {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { val: 0.0 }
    }
}

impl Pdf for StaticNoPdf {
    fn value(&self, _: &Vec3) -> f64 {
        0.0
    }
    fn generate(&self) -> Vec3 {
        Vec3::zero()
    }
}


//perlin.rs


const POINT_COUNT: usize = 256;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Perlin {
    pub ranvec: [Vec3; POINT_COUNT],
    pub ranfloat: [f64; POINT_COUNT],
    pub perm_x: [i32; POINT_COUNT],
    pub perm_y: [i32; POINT_COUNT],
    pub perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut raanv: [Vec3; POINT_COUNT] = [Vec3::zero(); POINT_COUNT];
        let mut rancopy: [f64; POINT_COUNT] = [0.0; POINT_COUNT];
        let mut perx: [i32; POINT_COUNT] = [0; POINT_COUNT];
        let mut pery: [i32; POINT_COUNT] = [0; POINT_COUNT];
        let mut perz: [i32; POINT_COUNT] = [0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            rancopy[i] = random_doouble();
            raanv[i] = Vec3::unit(Vec3::randomrange(-1.0, 1.0));
        }
        Perlin::perline_generate_perm(&mut perx);
        Perlin::perline_generate_perm(&mut pery);
        Perlin::perline_generate_perm(&mut perz);

        Self {
            ranvec: raanv,
            ranfloat: rancopy,
            perm_x: perx,
            perm_y: pery,
            perm_z: perz,
        }
    }
    #[allow(clippy::needless_range_loop)]
    #[allow(clippy::manual_swap)]
    pub fn perline_generate_perm(p: &mut [i32; POINT_COUNT]) {
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }
        // Perlin::permute(&mut p, POINT_COUNT as i32);

        let n = POINT_COUNT;
        for i in n - 1..0 {
            let axis = rand::thread_rng().gen_range(0..i);
            let tmp = p[i as usize];
            p[i as usize] = p[axis as usize];
            p[axis as usize] = tmp;
        }
    }
    #[allow(dead_code)]
    #[allow(clippy::needless_range_loop)]
    #[allow(clippy::manual_swap)]
    pub fn permute(arr: &mut [i32; POINT_COUNT], n: i32) {
        for i in n - 1..0 {
            let axis = rand::thread_rng().gen_range(0..i);
            let tmp = arr[i as usize];
            arr[i as usize] = arr[axis as usize];
            arr[axis as usize] = tmp;
        }
    }
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::needless_range_loop)]
    #[allow(clippy::manual_swap)]
    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        // u=u*u*(3.0-2.0*u);
        // v=v*v*(3.0-2.0*v);
        // w=w*w*(3.0-2.0*w);

        let i = (p.x.floor()) as i32;
        let j = (p.y.floor()) as i32;
        let k = (p.z.floor()) as i32;
        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[((self.perm_x[((i + (di as i32)) & 255) as usize]
                        as usize)
                        ^ (self.perm_y[((j + (dj as i32)) & 255) as usize] as usize)
                        ^ (self.perm_z[((k + (dk as i32)) & 255) as usize] as usize))
                        as usize];
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }
    #[allow(clippy::needless_range_loop)]
    #[allow(clippy::manual_swap)]
    pub fn trilinear_interp(my_sz: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - (i as f64), v - (j as f64), w - (k as f64));

                    accum += ((i as f64) * uu + ((1 - i) as f64) * (1.0 - uu))
                        * ((j as f64 * vv) + ((1 - j) as f64) * (1.0 - vv))
                        * ((k as f64) * ww + ((1 - k) as f64) * (1.0 - ww))
                        * (Vec3::dot(my_sz[i][j][k], weight_v));
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: Vec3) -> f64 {
        let depth = 7;
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * Perlin::noise(&self, temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * ((self.scale * p.z + 10.0 * self.noise.turb(*p)).sin() + 1.0)
    }
}

//ray.rs

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Ray {
    pub ori: Vec3,
    pub dic: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn new(ori: Vec3, dic: Vec3, tm: f64) -> Self {
        Self { ori, dic, tm }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.ori + self.dic * t
    }
}



//material.rs

fn schlick(cosin: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosin) * (1.0 - cosin) * (1.0 - cosin) * (1.0 - cosin) * (1.0 - cosin)
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> ScatterRecord;
    //attenuation是衰减的意思
    fn emitted(&self, _: &Hitrecord, _: f64, _: f64, _: &Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scattering_odf(&self, _: &Ray, _: &Hitrecord, _: &Ray) -> f64 {
        0.0
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Vec3,
    pub pdf_ptr: Arc<dyn Pdf>,
    pub isget: bool,
}

impl Lambertian {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo: Arc::new(BaseColor::new(albedo)),
        }
    }
    #[allow(dead_code)]
    pub fn new1(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &Ray,
        rec: &Hitrecord,
        _: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> ScatterRecord {
        //println!("{} {}", rec.v,rec.u);
        ScatterRecord {
            specular_ray: Ray::new(Vec3::zero(), Vec3::zero(), 0.0),
            is_specular: false,
            attenuation: Vec3 {
                x: self.albedo.value(rec.u, rec.v, &rec.p).x,
                y: self.albedo.value(rec.u, rec.v, &rec.p).y,
                z: self.albedo.value(rec.u, rec.v, &rec.p).z,
            },
            pdf_ptr: Arc::new(CosinePdf::new(&rec.normal)),
            isget: true,
        }
    }
    fn scattering_odf(&self, _: &Ray, rec: &Hitrecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal, scattered.dic.unit());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3, mut fuzz: f64) -> Self {
        if fuzz < 1.0 {
        } else {
            fuzz = 1.0
        }
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        _: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> ScatterRecord {
        let reflected = Vec3::reflect(r_in.dic, rec.normal);

        ScatterRecord {
            specular_ray: Ray::new(
                rec.p,
                reflected + Vec3::random_in_unit_sphere() * self.fuzz,
                0.0,
            ),
            is_specular: true,
            attenuation: self.albedo,
            pdf_ptr: Arc::new(NoPdf::new()),
            isget: true,
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    #[allow(dead_code)]
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    #[allow(clippy::needless_return)]
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> ScatterRecord {
        let mut srec: ScatterRecord = ScatterRecord {
            specular_ray: Ray {
                ori: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                dic: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tm: 0.0,
            },
            is_specular: false,
            attenuation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            pdf_ptr: Arc::new(NoPdf::new()),
            isget: true,
        };
        srec.is_specular = true;
        srec.pdf_ptr = Arc::new(NoPdf::new());
        srec.attenuation = Vec3::new(1.0, 1.0, 1.0);
        srec.isget = true;

        // attenuation=Vec3::new(1.0,1.0,1.0);
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0; //glass dont absorb ray so the attenuation is constly 1

        let etai_over_etat: f64;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx
        } else {
            etai_over_etat = self.ref_idx
        }

        let unit_direction = Vec3::unit(r_in.dic);
        let cos_theta: f64;
        if Vec3::dot(-unit_direction, rec.normal) < 1.0 {
            cos_theta = Vec3::dot(-unit_direction, rec.normal)
        } else {
            cos_theta = 1.0
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);

            srec.specular_ray = Ray::new(rec.p, reflected, r_in.tm);
            return srec;
        }
        let reflect_pro = schlick(cos_theta, etai_over_etat);
        if random_doouble() < reflect_pro {
            let reflected = Vec3::reflect(unit_direction, rec.normal);

            srec.specular_ray = Ray::new(rec.p, reflected, r_in.tm);

            return srec;
        }

        let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        srec.specular_ray = Ray::new(rec.p, refracted, r_in.tm);
        return srec;
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

#[allow(dead_code)]
impl DiffuseLight {
    pub fn new(c: Vec3) -> Self {
        Self {
            emit: Arc::new(BaseColor::new(c)),
        }
    }
    pub fn new1(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _: &Ray,
        _: &Hitrecord,
        _: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> ScatterRecord {
        ScatterRecord {
            specular_ray: Ray {
                ori: Vec3::zero(),
                dic: Vec3::zero(),
                tm: 0.0,
            },
            is_specular: false,
            attenuation: Vec3::zero(),
            pdf_ptr: Arc::new(NoPdf::new()),
            isget: false,
        }
    }
    #[allow(clippy::needless_return)]
    fn emitted(&self, rec: &Hitrecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
        if rec.front_face {
            return self.emit.value(u, v, p);
        }

        return self.emit.value(u, v, p);
    }
}

#[derive(Clone)]
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    #[allow(dead_code)]
    pub fn new(c: Vec3) -> Self {
        Self {
            albedo: Arc::new(BaseColor::new(c)),
        }
    }
    #[allow(dead_code)]
    pub fn new1(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &Hitrecord,
        attenuation: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> ScatterRecord {
        let temp = self.albedo.value(rec.u, rec.v, &rec.p);
        attenuation.x = temp.x;
        attenuation.y = temp.y;
        attenuation.z = temp.z;
        ScatterRecord {
            specular_ray: Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.tm),
            is_specular: false,
            attenuation: Vec3::new(temp.x, temp.y, temp.z),
            pdf_ptr: Arc::new(NoPdf::new()),
            isget: true,
        }
    }
    fn scattering_odf(&self, _: &Ray, rec: &Hitrecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal, scattered.dic.unit());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

#[derive(Clone)]
pub struct FlipFace {
    ptr: Arc<dyn Hittable>,
}

impl FlipFace {
    #[allow(dead_code)]
    pub fn new(ptr: Arc<dyn Hittable>) -> Self {
        Self { ptr }
    }
}

impl Hittable for FlipFace {
    #[allow(clippy::needless_return)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        if let Option::Some(mut rec) = self.ptr.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            return Some(rec);
        }
        return None;
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.ptr.bounding_box(time0, time1)
    }
}

pub trait StaticMaterial: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &StaticHitrecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> StaticScatterRecord;
    //attenuation是衰减的意思
    fn emitted(&self, _: &StaticHitrecord, _: f64, _: f64, _: &Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scattering_odf(&self, _: &Ray, _: &StaticHitrecord, _: &Ray) -> f64 {
        0.0
    }
}

#[derive(Clone)]
pub struct StaticLambertian<T: Texture> {
    albedo: T,
}

pub struct StaticScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Vec3,
    pub pdf_ptr: CosinePdf,
    pub isget: bool,
}

impl<T: Texture> StaticLambertian<T> {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3) -> StaticLambertian<StaticBaseColor> {
        StaticLambertian {
            albedo: StaticBaseColor::new(albedo),
        }
    }
    pub fn new1(albedo: T) -> StaticLambertian<T> {
        Self { albedo }
    }
}

impl<T: Texture> StaticMaterial for StaticLambertian<T> {
    fn scatter(
        &self,
        _: &Ray,
        rec: &StaticHitrecord,
        _: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> StaticScatterRecord {
        StaticScatterRecord {
            specular_ray: Ray::new(Vec3::zero(), Vec3::zero(), 0.0),
            is_specular: false,
            attenuation: Vec3 {
                x: self.albedo.value(rec.u, rec.v, &rec.p).x,
                y: self.albedo.value(rec.u, rec.v, &rec.p).y,
                z: self.albedo.value(rec.u, rec.v, &rec.p).z,
            },
            pdf_ptr: CosinePdf::new(&rec.normal),
            isget: true,
        }
    }
    fn scattering_odf(&self, _: &Ray, rec: &StaticHitrecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal, scattered.dic.unit());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct StaticMetal {
    albedo: Vec3,
    fuzz: f64,
}

impl StaticMetal {
    pub fn new(albedo: Vec3, mut fuzz: f64) -> Self {
        if fuzz < 1.0 {
        } else {
            fuzz = 1.0
        }
        Self { albedo, fuzz }
    }
}

impl StaticMaterial for StaticMetal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &StaticHitrecord,
        _: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> StaticScatterRecord {
        let reflected = Vec3::reflect(r_in.dic, rec.normal);

        StaticScatterRecord {
            specular_ray: Ray::new(
                rec.p,
                reflected + Vec3::random_in_unit_sphere() * self.fuzz,
                0.0,
            ),
            is_specular: true,
            attenuation: self.albedo,
            pdf_ptr: CosinePdf::new(&Vec3::zero()),
            isget: true,
        }
    }
}

#[derive(Clone)]
pub struct StaticDielectric {
    ref_idx: f64,
}

impl StaticDielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl StaticMaterial for StaticDielectric {
    #[allow(clippy::needless_return)]
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &StaticHitrecord,
        attenuation: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> StaticScatterRecord {
        let mut srec: StaticScatterRecord = StaticScatterRecord {
            specular_ray: Ray {
                ori: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                dic: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tm: 0.0,
            },
            is_specular: false,
            attenuation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            pdf_ptr: CosinePdf::new(&Vec3::zero()),
            isget: true,
        };
        srec.is_specular = true;
        srec.pdf_ptr = CosinePdf::new(&Vec3::zero());
        srec.attenuation = Vec3::new(1.0, 1.0, 1.0);
        srec.isget = true;

        // attenuation=Vec3::new(1.0,1.0,1.0);
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0; //glass dont absorb ray so the attenuation is constly 1

        let etai_over_etat: f64;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx
        } else {
            etai_over_etat = self.ref_idx
        }

        let unit_direction = Vec3::unit(r_in.dic);
        let cos_theta: f64;
        if Vec3::dot(-unit_direction, rec.normal) < 1.0 {
            cos_theta = Vec3::dot(-unit_direction, rec.normal)
        } else {
            cos_theta = 1.0
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);

            srec.specular_ray = Ray::new(rec.p, reflected, r_in.tm);
            return srec;
        }
        let reflect_pro = schlick(cos_theta, etai_over_etat);
        if random_doouble() < reflect_pro {
            let reflected = Vec3::reflect(unit_direction, rec.normal);

            srec.specular_ray = Ray::new(rec.p, reflected, r_in.tm);

            return srec;
        }

        let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        srec.specular_ray = Ray::new(rec.p, refracted, r_in.tm);
        return srec;
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct StaticDiffuseLight<T: Texture> {
    emit: T,
}

#[allow(dead_code)]
impl<T: Texture> StaticDiffuseLight<T> {
    pub fn new(c: Vec3) -> StaticDiffuseLight<StaticBaseColor> {
        StaticDiffuseLight {
            emit: StaticBaseColor::new(c),
        }
    }
    pub fn new1(emit: T) -> StaticDiffuseLight<T> {
        Self { emit }
    }
}

impl<T: Texture> StaticMaterial for StaticDiffuseLight<T> {
    fn scatter(
        &self,
        _: &Ray,
        _: &StaticHitrecord,
        _: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> StaticScatterRecord {
        StaticScatterRecord {
            specular_ray: Ray {
                ori: Vec3::zero(),
                dic: Vec3::zero(),
                tm: 0.0,
            },
            is_specular: false,
            attenuation: Vec3::zero(),
            pdf_ptr: CosinePdf::new(&Vec3::zero()),
            isget: false,
        }
    }
    #[allow(clippy::needless_return)]
    fn emitted(&self, rec: &StaticHitrecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
        if rec.front_face {
            return self.emit.value(u, v, p);
        }
        return self.emit.value(u, v, p);
    }
}

#[derive(Clone)]
pub struct StaticIsotropic<T: Texture> {
    pub(crate) albedo: T,
}

impl<T: Texture> StaticIsotropic<T> {
    #[allow(dead_code)]
    pub fn new(c: Vec3) -> StaticIsotropic<StaticBaseColor> {
        StaticIsotropic {
            albedo: StaticBaseColor::new(c),
        }
    }
    #[allow(dead_code)]
    pub fn new1(albedo: T) -> StaticIsotropic<T> {
        Self { albedo }
    }
}

impl<T: Texture> StaticMaterial for StaticIsotropic<T> {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &StaticHitrecord,
        attenuation: &mut Vec3,
        _: &mut Ray,
        _: &mut f64,
    ) -> StaticScatterRecord {
        let temp = self.albedo.value(rec.u, rec.v, &rec.p);
        attenuation.x = temp.x;
        attenuation.y = temp.y;
        attenuation.z = temp.z;
        StaticScatterRecord {
            specular_ray: Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.tm),
            is_specular: false,
            attenuation: Vec3::new(temp.x, temp.y, temp.z),
            pdf_ptr: CosinePdf::new(&Vec3::zero()),
            isget: true,
        }
    }
    fn scattering_odf(&self, _: &Ray, rec: &StaticHitrecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal, scattered.dic.unit());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}

#[derive(Clone)]
pub struct StaticFlipFace<T: StaticHittable> {
    ptr: T,
}

impl<T: StaticHittable> StaticFlipFace<T> {
    pub fn new(ptr: T) -> Self {
        Self { ptr }
    }
}

impl<T: StaticHittable> StaticHittable for StaticFlipFace<T> {
    #[allow(clippy::needless_return)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
        if let Option::Some(mut rec) = self.ptr.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            return Some(rec);
        }
        return None;
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.ptr.bounding_box(time0, time1)
    }
}

//hittable.rs

use std::option::Option::Some;
use std::sync::Arc;

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
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

// trait Hittable<'a> {
//     fn hit(&self);
// }
//
// trait Material {
//     fn result(&self);
// }
//
// struct Glass;
//
// impl Material for Glass {
//     fn result(&self) {
//         println!("Glass is broken!");
//     }
// }
//
// struct Sphere<'a> {
//     name: String,
//     mat_ptr: &'a dyn Material,
// }
//
// impl<'a> Hittable<'a> for Sphere<'a> {
//     fn hit(&self) {
//         println!("Name is {}", self.name);
//         self.mat_ptr.result();
//     }
// }
//
// struct HT<'a> {
//     pub objects: Vec<Box<dyn Hittable<'a>>>,
// }
//
// fn main() {
//     let mut list = HT { objects: vec![] };
//     let surface_material = Glass;
//     let s = Sphere {
//         name: String::from("球"),
//         mat_ptr: &surface_material,
//     };
//     list.objects.push(Box::new(s));
// }

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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        self.ptr.pdf_value(&(*o - self.offset), v)
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        self.ptr.random(&(*o - self.offset))
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
    #[allow(dead_code)]
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
    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let mut rotateo = Vec3::zero();
        rotateo.y = o.y;
        let mut rotatev = Vec3::zero();
        rotatev.y = v.y;
        rotateo.x = self.cos_theta * o.x - self.sin_theta * o.z;
        rotateo.z = self.sin_theta * o.x + self.cos_theta * o.z;
        rotatev.x = self.cos_theta * v.x - self.sin_theta * v.z;
        rotatev.z = self.sin_theta * v.x + self.cos_theta * v.z;
        self.ptr.pdf_value(&rotateo, &rotatev)
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let mut rotateo = Vec3::zero();
        rotateo.y = o.y;
        rotateo.x = self.cos_theta * o.x - self.sin_theta * o.z;
        rotateo.z = self.sin_theta * o.x + self.cos_theta * o.z;
        let rec = self.ptr.random(&rotateo);
        let mut ans = Vec3::zero();
        ans.y = rec.y;

        ans.x = self.cos_theta * rec.x - self.sin_theta * rec.z;
        ans.z = self.sin_theta * rec.x + self.cos_theta * rec.z;

        ans

        //self.ptr.random(&(*o - self.offset))
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
    #[allow(clippy::redundant_pattern_matching)]
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
    #[allow(dead_code)]
    #[allow(clippy::redundant_pattern_matching)]
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
    #[allow(dead_code)]
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
            if let Option::Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, INF) {
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
                let distangce_inside_boundary = (rec2.t - rec1.t) * ray_length;
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

pub trait StaticHittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
    fn pdf_value(&self, _: &Vec3, _: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
} //相当于一个基类 在列表里面会去看是谁将它实例化（如圆等图形）

#[derive(Clone)]
pub struct StaticHitrecord<'a> {
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
    pub mat_ptr: &'a dyn StaticMaterial,
}

impl<'a> StaticHitrecord<'a> {
    #[allow(dead_code)]
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
        mat_ptr: &'a dyn StaticMaterial,
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
pub struct StaticMovingSphere<T: StaticMaterial> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: T,
}

impl<T: StaticMaterial> StaticMovingSphere<T> {
    #[allow(dead_code)]
    pub fn new(cen0: Vec3, cen1: Vec3, _time0: f64, _time1: f64, r: f64, mat_ptr: T) -> Self {
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
impl<T: StaticMaterial + Clone> StaticHittable for StaticMovingSphere<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
        let oc = r.ori - StaticMovingSphere::center(self, r.tm);
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
            let mat_tem = &self.mat_ptr;
            let mut rec = StaticHitrecord {
                t: 0.0,
                u: 0.0,
                p: Vec3::zero(),
                normal: Vec3::zero(),
                front_face: false,
                mat_ptr: mat_tem,
                v: 0.0,
            };

            rec.t = root;
            rec.p = Ray::at(&r, rec.t);
            let outward_normal = (rec.p - StaticMovingSphere::center(self, r.tm)) / self.radius;
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

pub struct StaticSphere<T: StaticMaterial> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: T,
}

impl<T: StaticMaterial> StaticSphere<T> {
    #[allow(dead_code)]
    pub fn new(p: Vec3, normal: Vec3, t: f64, center: Vec3, radius: f64, mat_ptr: T) -> Self {
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
impl<T: StaticMaterial + Clone> StaticHittable for StaticSphere<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
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
            let mut rec = StaticHitrecord {
                t: 0.0,
                u: 0.0,
                p: Vec3::zero(),
                normal: Vec3::zero(),
                front_face: false,
                mat_ptr: &self.mat_ptr,
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

pub struct StaticBox1<T: StaticMaterial + Clone> {
    pub(crate) box_min: Vec3,
    pub(crate) box_max: Vec3,
    pub(crate) sides: (
        StaticXyRect<T>,
        StaticXyRect<T>,
        StaticYzRect<T>,
        StaticYzRect<T>,
        StaticXzRect<T>,
        StaticXzRect<T>,
    ),
}

impl<T: StaticMaterial + Clone> StaticHittable for StaticBox1<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
        let mut the_closest = t_max;
        let mut ans: Option<StaticHitrecord> = None;
        if let Some(rec) = self.sides.0.hit(r, t_min, t_max) {
            ans = Some(rec.clone());
            the_closest = rec.t;
        }
        if let Some(rec) = self.sides.1.hit(r, the_closest, t_max) {
            ans = Some(rec.clone());
            the_closest = rec.t;
        }
        if let Some(rec) = self.sides.2.hit(r, the_closest, t_max) {
            ans = Some(rec.clone());
            the_closest = rec.t;
        }
        if let Some(rec) = self.sides.3.hit(r, the_closest, t_max) {
            ans = Some(rec.clone());
            the_closest = rec.t;
        }
        if let Some(rec) = self.sides.4.hit(r, the_closest, t_max) {
            ans = Some(rec.clone());
            the_closest = rec.t;
        }
        if let Some(rec) = self.sides.5.hit(r, the_closest, t_max) {
            ans = Some(rec.clone());
        }
        ans
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Option::from(Aabb::new(self.box_min, self.box_max))
    }
}

impl<T: StaticMaterial + Clone> StaticBox1<T> {
    pub fn new(p0: &Vec3, p1: &Vec3, ptr: T) -> Self {
        Self {
            box_min: *p0,
            box_max: *p1,

            sides: (
                StaticXyRect {
                    mp: ptr.clone(),
                    x0: p0.x,
                    x1: p1.x,
                    y0: p0.y,
                    y1: p1.y,
                    k: p1.z,
                },
                StaticXyRect {
                    mp: ptr.clone(),
                    x0: p0.x,
                    x1: p1.x,
                    y0: p0.y,
                    y1: p1.y,
                    k: p0.z,
                },
                StaticYzRect {
                    mp: ptr.clone(),
                    y0: p1.y,
                    y1: p0.y,
                    z0: p1.z,
                    z1: p1.z,
                    k: p1.x,
                },
                StaticYzRect {
                    mp: ptr.clone(),
                    y0: p1.y,
                    y1: p0.y,
                    z0: p1.z,
                    z1: p1.z,
                    k: p0.x,
                },
                StaticXzRect {
                    mp: ptr.clone(),
                    x0: p1.x,
                    x1: p0.x,
                    z0: p1.z,
                    z1: p0.z,
                    k: p0.y,
                },
                StaticXzRect {
                    mp: ptr.clone(),
                    x0: p1.x,
                    x1: p0.x,
                    z0: p1.z,
                    z1: p0.z,
                    k: p1.y,
                },
            ),
        }
    }
}

pub struct StaticTranslate<T: StaticHittable> {
    pub(crate) ptr: T,
    pub(crate) offset: Vec3,
}

impl<T: StaticHittable> StaticTranslate<T> {
    pub fn new(ptr: T, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}

impl<T: StaticHittable> StaticHittable for StaticTranslate<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
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
pub struct StaticRotateY<T: StaticHittable> {
    //相对观察视角物体旋转的角度
    pub(crate) ptr: T,
    pub(crate) sin_theta: f64,
    pub(crate) cos_theta: f64,
    pub(crate) hasbox: bool,
    pub(crate) bbox: Aabb,
}

impl<T: StaticHittable> StaticRotateY<T> {
    pub fn new(p: T, angle: f64) -> Self {
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

impl<T: StaticHittable> StaticHittable for StaticRotateY<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
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
pub struct StaticRotateX<T: StaticHittable> {
    //相对观察视角物体旋转的角度
    pub(crate) ptr: T,
    pub(crate) sin_theta: f64,
    pub(crate) cos_theta: f64,
    pub(crate) hasbox: bool,
    pub(crate) bbox: Aabb,
}

impl<T: StaticHittable> StaticRotateX<T> {
    #[allow(dead_code)]
    #[allow(clippy::redundant_pattern_matching)]
    pub fn new(p: T, angle: f64) -> Self {
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

impl<T: StaticHittable> StaticHittable for StaticRotateX<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
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
pub struct StaticRotateZ<T: StaticHittable> {
    //相对观察视角物体旋转的角度
    pub(crate) ptr: T,
    pub(crate) sin_theta: f64,
    pub(crate) cos_theta: f64,
    pub(crate) hasbox: bool,
    pub(crate) bbox: Aabb,
}

impl<T: StaticHittable> StaticRotateZ<T> {
    #[allow(dead_code)]
    #[allow(clippy::redundant_pattern_matching)]
    pub fn new(p: T, angle: f64) -> Self {
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

impl<T: StaticHittable> StaticHittable for StaticRotateZ<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
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

unsafe impl Send for StaticHittableList {}

unsafe impl Sync for StaticHittableList {}

pub struct StaticHittableList {
    pub objects: Vec<Arc<dyn StaticHittable>>,
    //传出bool值可以用引用传递，先完善hittable 和add 函数
}

impl StaticHittableList {
    pub fn add(&mut self, object: Arc<dyn StaticHittable>) {
        self.objects.push(object);
    }
}

impl StaticHittable for StaticHittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
        let mut rec: Option<StaticHitrecord> = Option::None;
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

pub struct StaticConstantMedium<T1: StaticHittable, T2: Clone + StaticMaterial> {
    pub boundary: T1,
    pub phase_function: T2,
    neg_inv_density: f64,
}

#[allow(clippy::needless_return)]
impl<T1: StaticHittable, T2: Clone + StaticMaterial> StaticHittable
for StaticConstantMedium<T1, T2>
{
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
        if let Option::Some(mut rec1) = self.boundary.hit(r, -INF, INF) {
            if let Option::Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, INF) {
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
                let distangce_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_doouble().ln();

                if hit_distance > distangce_inside_boundary {
                    return None;
                }
                let mut recreturn = StaticHitrecord::new(
                    Vec3::zero(),
                    Vec3::zero(),
                    0.0,
                    false,
                    &self.phase_function,
                );
                recreturn.t = rec1.t + hit_distance / ray_length;
                recreturn.p = r.at(recreturn.t);
                recreturn.normal = Vec3::new(1.0, 0.0, 0.0);
                recreturn.front_face = true;
                recreturn.mat_ptr = &self.phase_function;
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

impl<T1: StaticHittable, T2: Clone + Texture>
StaticConstantMedium<T1, StaticIsotropic<T2>>
{
    #[allow(dead_code)]
    pub fn new(b: T1, d: f64, c: T2) -> Self {
        //c 用vec来new出一个basrcolor !!!!
        Self {
            boundary: b,
            phase_function: StaticIsotropic { albedo: c },
            neg_inv_density: (-1.0 / d),
        }
    }
}

pub struct StaticBvhNode<T1: StaticHittable, T2: StaticHittable> {
    pub left: Arc<T1>,
    pub right: Arc<T2>,
    pub box1: Aabb,
}

impl<T1: StaticHittable, T2: StaticHittable> StaticBvhNode<T1, T2> {
    #[allow(dead_code)]
    pub fn new(left: Arc<T1>, right: Arc<T2>, time0: f64, time1: f64) -> Self {
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
impl<T1: StaticHittable, T2: StaticHittable> StaticHittable for StaticBvhNode<T1, T2> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
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


//vec3.rs
use rand::Rng;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
//use proc_macro::{TokenStream, Ident, Span};

//let secret_number = ;


#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// impl quote::ToTokens for Vec3 {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         tokens.append(Ident::new("Vec3", Span::call_site()));
//     }
// }

#[warn(unused_parens)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    #[allow(dead_code)]
    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        { (self.x * self.x + self.y * self.y + self.z * self.z) as f64 }.sqrt()
    }

    pub fn unit(self) -> Self {
        Self::new(
            self.x / self.length(),
            self.y / self.length(),
            self.z / self.length(),
        )
    }

    pub fn dot(a: Self, b: Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn random() -> Self {
        Self::new(random_doouble(), random_doouble(), random_doouble())
    }
    pub fn randomrange(min: f64, max: f64) -> Self {
        Self::new(
            range_random_double(min, max),
            range_random_double(min, max),
            range_random_double(min, max),
        )
    }
    #[allow(dead_code)]
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        (self.x).abs() < s && self.y.abs() < s && self.z.abs() < s
    }
    //may need to do
    pub fn reflect(v: Self, n: Self) -> Self {
        v - n * (2.0_f64 * Vec3::dot(v, n))
    }
    pub fn refract(uv: Self, n: Self, etai_over_etat: f64) -> Self {
        let costhta = Vec3::dot(-uv, n);
        let r_out_perp = (uv + n * costhta) * etai_over_etat;
        let r_out_parallel = n * (-(1.0 - Vec3::squared_length(&r_out_perp)).abs().sqrt());
        r_out_perp + r_out_parallel
    }
    pub fn cross(u: Self, v: Self) -> Self {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }
    pub fn sameside(a: Vec3, b: Vec3, c: Vec3, p: Vec3) -> bool {
        let ab = b - a;
        let ac = c - a;
        let ap = p - a;

        let v1 = Vec3::cross(ab, ac);
        let v2 = Vec3::cross(ab, ap);
        Vec3::dot(v1, v2) >= 0.0
    }
    #[allow(clippy::needless_return)]
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                range_random_double(-1.0, 1.0),
                range_random_double(-1.0, 1.0),
                0.0,
            );
            if p.squared_length() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

impl Vec3 {
    pub fn get(&self, demesion: i32) -> f64 {
        if demesion == 0 {
            self.x
        } else if demesion == 1 {
            self.y
        } else {
            self.z
        }
    }
    #[allow(dead_code)]
    pub fn random_unit_vector() -> Vec3 {
        let a = range_random_double(0.0, (2.0 * PI) as f64);
        let z = range_random_double(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }
    #[allow(clippy::needless_return)]
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::randomrange(-1.0, 1.0);
            if p.squared_length() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    #[allow(dead_code)]
    pub fn random_in_himisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
    pub fn random_cosine_direction() -> Vec3 {
        let r1 = random_doouble();
        let r2 = random_doouble();
        let z = (1.0 - r2).sqrt();
        let phi = 2.0 * PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();
        Vec3::new(x, y, z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }

    #[test]
    fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + 233.0,
            Vec3::new(234.0, 233.0, 232.0)
        )
    }

    /*
    #[test]
    fn test_add_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += 233.0;
        assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(-1.0, -4.0, -7.0)
        )
    }

    #[test]
    fn test_sub_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    }

    #[test]
    fn test_sub_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_sub_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= 1.0;
        assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x *= 2.0;
        assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }

    #[test]
    fn test_elemul() {
        assert_eq!(
            Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        );
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }
    */

    // #[test]
    // fn test_squared_length() {
    //     assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0_f64);
    // }

    /*
    #[test]
    fn test_length() {
        assert_eq!(
            Vec3::new(3.0, 4.0, 5.0).length(),
            ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
        );
    }

    #[test]
    fn test_unit() {
        assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(
            Vec3::new(-233.0, 0.0, 0.0).unit(),
            Vec3::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_unit_panic() {
        Vec3::new(0.0, 0.0, 0.0).unit();
    }
    */
}



//aabb.rs

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
                if t0 < t_min {
                } else {
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
                if t0 < t_min {
                } else {
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
                if t0 < t_min {
                } else {
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



#[allow(clippy::needless_return)]
pub fn maxnum1(a: f64, b: f64, c: f64) -> f64 {
    if a < b {
        if c < b {
            b
        } else {
            c
        }
    } else if a < c {
        c
    } else {
        a
    }
}

#[allow(clippy::needless_return)]
pub fn mainnum1(a: f64, b: f64, c: f64) -> f64 {
    if a < b {
        if c < a {
            c
        } else {
            a
        }
    } else if b < c {
        b
    } else {
        c
    }
}

pub struct XyRect {
    pub(crate) mp: Arc<dyn Material>,
    pub(crate) x0: f64,
    pub(crate) x1: f64,
    pub(crate) y0: f64,
    pub(crate) y1: f64,
    pub(crate) k: f64,
}

#[allow(dead_code)]
impl XyRect {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        }
    }
}

impl Hittable for XyRect {
    #[allow(clippy::needless_return)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let t = (self.k - r.ori.z) / r.dic.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.ori.x + t * r.dic.x;
        let y = r.ori.y + t * r.dic.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let ourward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        if let Option::Some(rec) = self.hit(Ray::new(*o, *v, 0.0), 0.001, INFINITY) {
            let area = (self.x1 - self.x0) * (self.y1 - self.y0);
            let distance_squared = rec.t * rec.t * v.squared_length();
            let cosine = Vec3::dot(*v, rec.normal).abs() / v.length();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }
    #[allow(clippy::needless_return)]
    fn random(&self, o: &Vec3) -> Vec3 {
        let randompoint = Vec3::new(
            range_random_double(self.x0, self.x1),
            range_random_double(self.y0, self.y1),
            self.k,
        );
        randompoint - *o //分布在击中点球面上的一个点与球心的连线
    }
}

pub struct Triangel {
    pub(crate) mp: Arc<dyn Material>,
    pub a1: Vec3,
    pub a2: Vec3,
    pub a3: Vec3,
}

unsafe impl Send for Triangel {}

unsafe impl Sync for Triangel {}

impl Triangel {
    #[allow(dead_code)]
    pub fn new(_a1: Vec3, _a2: Vec3, _a3: Vec3, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            a1: _a1,
            a2: _a2,
            a3: _a3,
        }
    }
}

impl Hittable for Triangel {
    #[allow(clippy::needless_return)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let dirct1 = self.a2 - self.a1;
        let dirct2 = self.a3 - self.a1;
        let n = Vec3::cross(dirct1, dirct2);
        let b_a = self.a1 - r.ori;
        let t = Vec3::dot(n, b_a) / Vec3::dot(n, r.dic);
        //inspired by https://blog.csdn.net/wuwangrun/article/details/8188665
        if t < t_min || t > t_max {
            return None;
        }
        let hit = r.at(t);
        if Vec3::sameside(self.a1, self.a2, self.a3, hit)
            && Vec3::sameside(self.a2, self.a3, self.a1, hit)
            && Vec3::sameside(self.a3, self.a1, self.a2, hit)
        {
            //use the method 2 in https://www.cnblogs.com/graphics/archive/2010/08/05/1793393.html
            let mut rec = Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());
            rec.p = r.at(t);
            let a1 = self.a1.x - self.a2.x;
            let b1 = self.a1.x - self.a3.x;
            let c1 = self.a1.x - hit.x;
            let a2 = self.a1.y - self.a2.y;
            let b2 = self.a1.y - self.a3.y;
            let c2 = self.a1.y - hit.y;
            rec.u = (c1 * b2 - b1 * c2) / (a1 * b2 - b1 * a2);
            rec.v = (a1 * c2 - a2 * c1) / (a1 * b2 - b1 * a2); //may change the order //use the most stupid way to solve the problem
            //println!("{},{}",rec.u,rec.v);
            //the silly way
            // rec.v=0.0;
            // rec.u=0.0;//todo big problem!!!
            rec.t = t;
            let ourward_normal = n.unit();
            rec.set_face_normal(&r, ourward_normal);
            rec.mat_ptr = self.mp.clone();
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        // let dirct1 = self.a2 - self.a1;
        // let dirct2 = self.a3 - self.a1;
        // let n = Vec3::cross(dirct1, dirct2);
        let ans1 = Vec3::new(
            mainnum1(self.a1.x, self.a2.x, self.a3.x),
            mainnum1(self.a1.y, self.a2.y, self.a3.y),
            mainnum1(self.a1.z, self.a2.z, self.a3.z),
        ) + Vec3::new(0.01, 0.01, 0.01);
        let ans2 = Vec3::new(
            maxnum1(self.a1.x, self.a2.x, self.a3.x),
            maxnum1(self.a1.y, self.a2.y, self.a3.y),
            maxnum1(self.a1.z, self.a2.z, self.a3.z),
        ) - Vec3::new(0.01, 0.01, 0.01);

        Some(Aabb::new(ans1, ans2))
    }
}

pub struct XzRect {
    pub(crate) mp: Arc<dyn Material>,
    pub(crate) x0: f64,
    pub(crate) x1: f64,
    pub(crate) z0: f64,
    pub(crate) z1: f64,
    pub(crate) k: f64,
}

unsafe impl Send for XzRect {}

unsafe impl Sync for XzRect {}

impl XzRect {
    #[allow(dead_code)]
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }
}

impl Hittable for XzRect {
    #[allow(clippy::needless_return)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let t = (self.k - r.ori.y) / r.dic.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.ori.x + t * r.dic.x;
        let z = r.ori.z + t * r.dic.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let ourward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        if let Option::Some(rec) = self.hit(Ray::new(*o, *v, 0.0), 0.001, INFINITY) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = rec.t * rec.t * v.squared_length();
            let cosine = Vec3::dot(*v, rec.normal).abs() / v.length();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }
    #[allow(clippy::needless_return)]
    fn random(&self, o: &Vec3) -> Vec3 {
        let randompoint = Vec3::new(
            range_random_double(self.x0, self.x1),
            self.k,
            range_random_double(self.z0, self.z1),
        );
        randompoint - *o //分布在击中点球面上的一个点与球心的连线
    }
}

#[allow(dead_code)]
pub struct YzRect {
    pub(crate) mp: Arc<dyn Material>,
    pub(crate) y0: f64,
    pub(crate) y1: f64,
    pub(crate) z0: f64,
    pub(crate) z1: f64,
    pub(crate) k: f64,
}

impl YzRect {
    #[allow(dead_code)]
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat,
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<Hitrecord> {
        let t = (self.k - r.ori.x) / r.dic.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.ori.y + t * r.dic.y;
        let z = r.ori.z + t * r.dic.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = Hitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, self.mp.clone());

        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let ourward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        if let Option::Some(rec) = self.hit(Ray::new(*o, *v, 0.0), 0.001, INFINITY) {
            let area = (self.y1 - self.y0) * (self.z1 - self.z0);
            let distance_squared = rec.t * rec.t * v.squared_length();
            let cosine = Vec3::dot(*v, rec.normal).abs() / v.length();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }
    #[allow(clippy::needless_return)]
    fn random(&self, o: &Vec3) -> Vec3 {
        let randompoint = Vec3::new(
            self.k,
            range_random_double(self.y0, self.y1),
            range_random_double(self.z0, self.z1),
        );
        randompoint - *o //分布在击中点球面上的一个点与球心的连线
    }
}

//static
pub struct StaticXyRect<T: StaticMaterial> {
    pub(crate) mp: T,
    pub(crate) x0: f64,
    pub(crate) x1: f64,
    pub(crate) y0: f64,
    pub(crate) y1: f64,
    pub(crate) k: f64,
}

#[allow(dead_code)]
impl<T: StaticMaterial> StaticXyRect<T> {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: T) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        }
    }
}

impl<T: StaticMaterial + Clone> StaticHittable for StaticXyRect<T> {
    #[allow(clippy::needless_return)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
        let t = (self.k - r.ori.z) / r.dic.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.ori.x + t * r.dic.x;
        let y = r.ori.y + t * r.dic.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let mut rec = StaticHitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, &self.mp);

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let ourward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.p = r.at(t);
        rec.mat_ptr = &self.mp;
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct StaticTriangel<T: StaticMaterial> {
    pub(crate) mp: T,
    pub a1: Vec3,
    pub a2: Vec3,
    pub a3: Vec3,
}

unsafe impl<T: StaticMaterial> Send for StaticTriangel<T> {}

unsafe impl<T: StaticMaterial> Sync for StaticTriangel<T> {}

impl<T: StaticMaterial> StaticTriangel<T> {
    #[allow(dead_code)]
    pub fn new(_a1: Vec3, _a2: Vec3, _a3: Vec3, mat: T) -> Self {
        Self {
            mp: mat,
            a1: _a1,
            a2: _a2,
            a3: _a3,
        }
    }
}

impl<T: StaticMaterial + Clone + Material> StaticHittable for StaticTriangel<T> {
    #[allow(clippy::needless_return)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
        let dirct1 = self.a2 - self.a1;
        let dirct2 = self.a3 - self.a1;
        let n = Vec3::cross(dirct1, dirct2);
        let b_a = self.a1 - r.ori;
        let t = Vec3::dot(n, b_a) / Vec3::dot(n, r.dic);
        //inspired by https://blog.csdn.net/wuwangrun/article/details/8188665
        if t < t_min || t > t_max {
            return None;
        }
        let hit = r.at(t);
        if Vec3::sameside(self.a1, self.a2, self.a3, hit)
            && Vec3::sameside(self.a2, self.a3, self.a1, hit)
            && Vec3::sameside(self.a3, self.a1, self.a2, hit)
        {
            //use the method 2 in https://www.cnblogs.com/graphics/archive/2010/08/05/1793393.html
            let mut rec = StaticHitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, &self.mp);
            rec.p = r.at(t);
            // let a1 = self.a1.x - self.a2.x;
            // let b1 = self.a1.x - self.a3.x;
            // let c1 = self.a1.x - hit.x;
            // let a2 = self.a1.y - self.a2.y;
            // let b2 = self.a1.y - self.a3.y;
            // let c2 = self.a1.y - hit.y;
            // rec.u=(c1*b2-b1*c2)/(a1*b2-b1*a2);
            // rec.v=(a1*c2-a2*c1)/(a1*b2-b1*a2);//may change the order //use the most stupid way to solve the problem
            // //the silly way
            rec.t = t;
            let ourward_normal = n;
            rec.set_face_normal(&r, ourward_normal);
            rec.mat_ptr = &self.mp;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        // let dirct1 = self.a2 - self.a1;
        // let dirct2 = self.a3 - self.a1;
        // let n = Vec3::cross(dirct1, dirct2);
        let ans1 = Vec3::new(
            mainnum1(self.a1.x, self.a2.x, self.a3.x),
            mainnum1(self.a1.y, self.a2.y, self.a3.y),
            mainnum1(self.a1.z, self.a2.z, self.a3.z),
        ) + Vec3::new(0.01, 0.01, 0.01);
        let ans2 = Vec3::new(
            maxnum1(self.a1.x, self.a2.x, self.a3.x),
            maxnum1(self.a1.y, self.a2.y, self.a3.y),
            maxnum1(self.a1.z, self.a2.z, self.a3.z),
        ) - Vec3::new(0.01, 0.01, 0.01);

        Some(Aabb::new(ans1, ans2))
    }
}

pub struct StaticXzRect<T: StaticMaterial> {
    pub(crate) mp: T,
    pub(crate) x0: f64,
    pub(crate) x1: f64,
    pub(crate) z0: f64,
    pub(crate) z1: f64,
    pub(crate) k: f64,
}

unsafe impl<T: StaticMaterial> Send for StaticXzRect<T> {}

unsafe impl<T: StaticMaterial> Sync for StaticXzRect<T> {}

impl<T: StaticMaterial> StaticXzRect<T> {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, mat: T) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }
}

impl<T: StaticMaterial + Clone> StaticHittable for StaticXzRect<T> {
    #[allow(clippy::needless_return)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
        let t = (self.k - r.ori.y) / r.dic.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.ori.x + t * r.dic.x;
        let z = r.ori.z + t * r.dic.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = StaticHitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, &self.mp);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let ourward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr = &self.mp;
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        if let Option::Some(rec) = self.hit(Ray::new(*o, *v, 0.0), 0.001, INFINITY) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = rec.t * rec.t * v.squared_length();
            let cosine = Vec3::dot(*v, rec.normal).abs() / v.length();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }
    #[allow(clippy::needless_return)]
    fn random(&self, o: &Vec3) -> Vec3 {
        let randompoint = Vec3::new(
            range_random_double(self.x0, self.x1),
            self.k,
            range_random_double(self.z0, self.z1),
        );
        randompoint - *o
    }
}

#[allow(dead_code)]
pub struct StaticYzRect<T: StaticMaterial> {
    pub(crate) mp: T,
    pub(crate) y0: f64,
    pub(crate) y1: f64,
    pub(crate) z0: f64,
    pub(crate) z1: f64,
    pub(crate) k: f64,
}

impl<T: StaticMaterial> StaticYzRect<T> {
    #[allow(dead_code)]
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, mat: T) -> Self {
        Self {
            mp: mat,
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }
}

impl<T: StaticMaterial + Clone> StaticHittable for StaticYzRect<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<StaticHitrecord> {
        let t = (self.k - r.ori.x) / r.dic.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.ori.y + t * r.dic.y;
        let z = r.ori.z + t * r.dic.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = StaticHitrecord::new(Vec3::zero(), Vec3::zero(), 0.0, false, &self.mp);

        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let ourward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(&r, ourward_normal);
        rec.mat_ptr = &self.mp;
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}


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
    degrees * PI / 180.0
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
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
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            range_random_double(self.time0, self.time1),
        )
    }
}

pub(crate) fn random_doouble() -> f64 {
    rand::thread_rng().gen_range(1..101) as f64 / 102.0
}

pub(crate) fn range_random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random_doouble()
}

#[allow(clippy::needless_return)]
pub(crate) fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    return x;
}

#[allow(clippy::needless_return)]
#[allow(dead_code)]
fn color(
    x: Ray,
    background: Vec3,
    world: &HittableList,
    lights: &Arc<dyn Hittable + Send>,
    dep: u32,
) -> Vec3 {
    if dep == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Option::Some(_rec) = world.hit(x, 0.001, INF) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
        let emitted = _rec.mat_ptr.emitted(&_rec, _rec.u, _rec.v, &_rec.p);
        let mut pdf_val = 0.0;
        let mut aldedo = Vec3::zero();
        let scatterrecord =
            _rec.mat_ptr
                .scatter(&x, &_rec, &mut aldedo, &mut scattered, &mut pdf_val);

        if scatterrecord.isget {
            if scatterrecord.is_specular {
                return scatterrecord.attenuation
                    * color(
                    scatterrecord.specular_ray,
                    background,
                    world,
                    lights,
                    dep - 1,
                );
            }

            let lightptr = Arc::new(HittablePdf::new(lights.clone(), &_rec.p));
            let p = MixturePdf::new(lightptr, scatterrecord.pdf_ptr);
            scattered = Ray::new(_rec.p, p.generate(), x.tm);

            pdf_val = p.value(&scattered.dic);

            let mm = emitted
                + scatterrecord.attenuation
                * _rec.mat_ptr.scattering_odf(&x, &_rec, &scattered)
                * color(scattered, background, world, lights, dep - 1)
                / pdf_val;

            return mm;
        }
        emitted
    } else {
        background
    }
}



#[allow(dead_code)]
#[allow(clippy::needless_return)]
fn staticcolor<T: StaticHittable>(
    x: Ray,
    background: Vec3,
    world: &StaticHittableList,
    lights: &T,
    dep: u32,
) -> Vec3 {
    if dep == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Option::Some(_rec) = world.hit(x, 0.001, INF) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
        let emitted = _rec.mat_ptr.emitted(&_rec, _rec.u, _rec.v, &_rec.p);
        let mut pdf_val = 0.0;
        let mut aldedo = Vec3::zero();
        let scatterrecord =
            _rec.mat_ptr
                .scatter(&x, &_rec, &mut aldedo, &mut scattered, &mut pdf_val);

        if scatterrecord.isget {
            if scatterrecord.is_specular {
                return scatterrecord.attenuation
                    * staticcolor(
                    scatterrecord.specular_ray,
                    background,
                    world,
                    lights,
                    dep - 1,
                );
            }

            let lightptr = StaticHittablePdf::new(lights, &_rec.p);
            let p = StaticMixturePdf::new(&lightptr, &scatterrecord.pdf_ptr);
            scattered = Ray::new(_rec.p, p.generate(), x.tm);

            pdf_val = p.value(&scattered.dic);

            let mm = emitted
                + scatterrecord.attenuation
                * _rec.mat_ptr.scattering_odf(&x, &_rec, &scattered)
                * staticcolor(scattered, background, world, lights, dep - 1)
                / pdf_val;

            return mm;
        }
        emitted
    } else {
        background
    }
}




pub fn criterion_benchmark(c: &mut Criterion) {
    let is_ci = match std::env::var("CI") {
        Ok(x) => x == "true",
        Err(_) => false,
    };
    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (16, 8) };
    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );
    //image
    let mut ratio: f64 = 1.0;
    // let image_width = 400 as u32;
    let mut image_width = 600_u32;
    let mut image_heigth = (image_width as f64 / ratio) as u32;
    let sample_per_pixel = 10; //ought to be 100  可以做的更大比如500//
    let max_depth = 50; //an bo modifyed to lessen the time
    let mut backgroud = Vec3::new(0.0, 0.0, 0.0);
    let mut lookfrom = Vec3::new(278.0, 278.0, -800.0); //13 2 3
    let mut lookat = Vec3::new(278.0, 278.0, 0.0);
    let mut vfov = 40.0;
    let number = 2;
    let mut world = StaticHittableList { objects: vec![] };
    match number {
        1 => {
            world = two_texture_static();
            vfov = 20.0;
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
        }
        2 => {
            world = static_random_sence();
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            vfov = 20.0;
            ratio = 3.0 / 2.0;

            image_width = 1200_u32;
            image_heigth = (image_width as f64 / ratio) as u32;
        }
        3 => {
            world = static_earth();
            vfov = 20.0;
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
        }
        4 => {
            world = static_cornell_box();
            backgroud = Vec3::new(0.0, 0.0, 0.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
        }
        5 => {
            world = static_bvh_random_scence();
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            vfov = 20.0;
            ratio = 3.0 / 2.0;
            image_width = 1200_u32;
            image_heigth = (image_width as f64 / ratio) as u32;
        }

        _ => println!("you are wrong !! please choose the wonderful world you want to see again."),
    }

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0; //ought to be 2
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    let mut img: RgbImage = ImageBuffer::new(image_width, image_heigth);
    let bar = ProgressBar::new(16);
let world_inthread = Arc::new(world);
        println!("yyy");
        let worldptr = world_inthread.clone();


    let mut lightworld: StaticHittableList = StaticHittableList { objects: vec![] };
            let light1: Arc<dyn StaticHittable + Send> = Arc::new(StaticXzRect::new(
                213.0,
                343.0,
                227.0,
                332.0,
                554.0,
                StaticLambertian::<StaticBaseColor>::new(Vec3::zero()),
            ));
            // let tmp: Arc<dyn StaticHittable + Send> = Arc::new(StaticSphere::new(
            //     Vec3::zero(),
            //     Vec3::zero(),
            //     0.0,
            //     Vec3::new(190.0, 90.0, 190.0),
            //     90.0,
            //     StaticLambertian::<StaticBaseColor>::new(Vec3::zero()),
            // ));
            lightworld.add(light1);
            //lightworld.add(tmp);
            let lightin = lightworld;
    for j in 0..image_heigth {
        for i in 0..image_width
        {
            let pixel = img.get_pixel_mut(i, j);
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..sample_per_pixel {
                let u = (i as f64 + random_doouble()) / (image_width - 1) as f64;
                let v = (image_heigth as f64 - j as f64 + random_doouble()) / (image_heigth - 1) as f64;
                let r = cam.get_ray(u, v);
                if i==50&&j==50{
                    c.bench_function("ray static ", |b| b.iter(|| staticcolor::<StaticHittableList>(black_box(r),black_box(backgroud),black_box(&worldptr),black_box(&lightin),black_box(max_depth))));
                }

                                    pixel_color += staticcolor::<StaticHittableList>(
                                        r, backgroud, &worldptr, &lightin, max_depth,
                                    );
            }

            let scale = 1.0 / sample_per_pixel as f64;
            let aaa = (pixel_color.x * scale).sqrt();
            let aaa1 = 256 as f64 * clamp(aaa, 0.0, 0.999);
            let bbb = (pixel_color.y * scale).sqrt();
            let bbb1 = 256 as f64 * clamp(bbb, 0.0, 0.999);
            let ccc = (pixel_color.z * scale).sqrt();
            let ccc1 = 256 as f64 * clamp(ccc, 0.0, 0.999);
            *pixel = image::Rgb([aaa1 as u8, bbb1 as u8, ccc1 as u8]);
        }
        bar.inc(1);
    }
    // C:\Users\18303\Desktop\learn\ppca\myraytracer1\output\testbench.png
    img.save("/mnt/c/Users/18303/Desktop/learn/ppca/myraytracer1/output/testbench.png").unwrap();
    bar.finish();
}



pub fn criterion_benchmark1(c: &mut Criterion) {
    let is_ci = match std::env::var("CI") {
        Ok(x) => x == "true",
        Err(_) => false,
    };
    let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (16, 8) };
    println!(
        "CI: {}, using {} jobs and {} workers",
        is_ci, n_jobs, n_workers
    );
    //image
    let mut ratio: f64 = 1.0;
    // let image_width = 400 as u32;
    let mut image_width = 600_u32;
    let mut image_heigth = (image_width as f64 / ratio) as u32;
    let sample_per_pixel = 10; //ought to be 100  可以做的更大比如500//
    let max_depth = 50; //an bo modifyed to lessen the time
    let mut backgroud = Vec3::new(0.0, 0.0, 0.0);
    let mut lookfrom = Vec3::new(278.0, 278.0, -800.0); //13 2 3
    let mut lookat = Vec3::new(278.0, 278.0, 0.0);
    let mut vfov = 40.0;
    //choose picture you want
    let number = 2;
    let mut world = HittableList { objects: vec![] };
    match number {
        1 => {
            world = two_spheres();
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            vfov = 20.0;
        }
        2 => {
            world = random_sence();
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            vfov = 20.0;
            ratio = 3.0 / 2.0;

            image_width = 1200_u32;
            image_heigth = (image_width as f64 / ratio) as u32;
        }
        3 => {
            world = two_berlin_spheres();
            vfov = 20.0;
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
        }
        4 => {
            world = earth();
            vfov = 20.0;
            lookat = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
        }
        5 => world = simple_light(),
        6 => {
            world = cornell_box();
            backgroud = Vec3::new(0.0, 0.0, 0.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
        }
        7 => world = cornell_smoke(),
        8 => {
            world = final_book2_scence();
            lookat = Vec3::new(278.0, 278.0, 0.0);
            lookfrom = Vec3::new(478.0, 278.0, -600.0); //todo
            vfov = 40.0;
            ratio = 1.0;
            image_width = 800_u32;
            image_heigth = (image_width as f64 / ratio) as u32;
        }
        9 => {
            world = my_scence_ball_world();
            lookat = Vec3::new(0.0, 0.0, 0.0);
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 0.0);
            vfov = 50.0;
        }
        10 => {
            world = obj();
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(60.0, 20.0, 30.0);
            lookat = Vec3::zero();
        }
        11 => world = obj_with_texture(),
        12 => {
            world = cornell_box_rabbit();
            backgroud = Vec3::new(0.0, 0.0, 0.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
        }
        13 => {
            world = my_world();
            backgroud = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(70.0, 0.0, 0.0);
            lookat = Vec3::zero();
            image_width = 1600_u32;
            ratio = 1.0 / 0.618;
            image_heigth = (image_width as f64 / ratio) as u32;
            vfov = 62.0;
        }
        14 => {
            println!("begin build scence");
            world = my_untimately();
            println!("finish build scence");

            backgroud = Vec3::new(0.7, 0.8, 1.0);
            lookat = Vec3::new(500.0, 278.0, 0.0);
            lookfrom = Vec3::new(500.0, 278.0, -800.0);
            image_width = 1600_u32;
            ratio = 1.0 / 0.618;
            image_heigth = (image_width as f64 / ratio) as u32;
        }
        _ => println!("you are wrong !! please choose the wonderful world you want to see again."),
    }
    let mut lightworld: HittableList = HittableList { objects: vec![] };
    let light1: Arc<dyn Hittable + Send> = Arc::new(XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::new(Lambertian::new(Vec3::zero())),
    ));

    lightworld.add(light1);
    let lights: Arc<dyn Hittable + Send> = Arc::new(lightworld);

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0; //ought to be 2
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    let mut img: RgbImage = ImageBuffer::new(image_width, image_heigth);
    let bar = ProgressBar::new(16);
    let world_inthread = Arc::new(world);
    println!("yyy");
    let worldptr = world_inthread.clone();


    // let mut lightworld: StaticHittableList = StaticHittableList { objects: vec![] };
    // let light1: Arc<dyn StaticHittable + Send> = Arc::new(StaticXzRect::new(
    //     213.0,
    //     343.0,
    //     227.0,
    //     332.0,
    //     554.0,
    //     StaticLambertian::<StaticBaseColor>::new(Vec3::zero()),
    // ));
    // // let tmp: Arc<dyn StaticHittable + Send> = Arc::new(StaticSphere::new(
    // //     Vec3::zero(),
    // //     Vec3::zero(),
    // //     0.0,
    // //     Vec3::new(190.0, 90.0, 190.0),
    // //     90.0,
    // //     StaticLambertian::<StaticBaseColor>::new(Vec3::zero()),
    // // ));
    // lightworld.add(light1);
    // //lightworld.add(tmp);
    // let lightin = lightworld;

    let lightsptr = lights.clone();
    for j in 0..image_heigth {
        for i in 0..image_width
        {
            let pixel = img.get_pixel_mut(i, j);
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..sample_per_pixel {
                let u = (i as f64 + random_doouble()) / (image_width - 1) as f64;
                let v = (image_heigth as f64 - j as f64 + random_doouble()) / (image_heigth - 1) as f64;
                let r = cam.get_ray(u, v);
                if i==50&&j==50{
                    c.bench_function("ray dynamically ", |b| b.iter(|| color(black_box(r),black_box(backgroud),black_box(&worldptr),black_box(&lightsptr),black_box(max_depth))));
                }
                pixel_color += color(r, backgroud, &worldptr, &lightsptr, max_depth);
                // pixel_color += staticcolor::<StaticHittableList>(
                //     r, backgroud, &worldptr, &lightin, max_depth,
                // );
            }

            let scale = 1.0 / sample_per_pixel as f64;
            let aaa = (pixel_color.x * scale).sqrt();
            let aaa1 = 256 as f64 * clamp(aaa, 0.0, 0.999);
            let bbb = (pixel_color.y * scale).sqrt();
            let bbb1 = 256 as f64 * clamp(bbb, 0.0, 0.999);
            let ccc = (pixel_color.z * scale).sqrt();
            let ccc1 = 256 as f64 * clamp(ccc, 0.0, 0.999);
            *pixel = image::Rgb([aaa1 as u8, bbb1 as u8, ccc1 as u8]);
        }
        bar.inc(1);
    }
    // C:\Users\18303\Desktop\learn\ppca\myraytracer1\output\testbench.png
    img.save("/mnt/c/Users/18303/Desktop/learn/ppca/myraytracer1/output/testbench.png").unwrap();
    bar.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
