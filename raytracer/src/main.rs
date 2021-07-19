#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod hittable;
mod camera;
mod rtweekend;
mod material;
mod aabb;
mod texture;
mod perlin;
mod aarect;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;
pub use crate::ray::Ray;
use crate::hittable::{HittableList, Sphere, Hittable, MovingSphere, Box1, RotateY, Translate, ConstantMedium};
use std::sync::Arc;
use std::f32::INFINITY;
use rand::Rng;
use crate::material::{Metal, Lambertian, Dielectric, DiffuseLight};
use std::f64::consts::PI;
use crate::texture::{CheckerTexture, Texture, ImageTexture, BaseColor};
use crate::perlin::NoiseTexture;
use crate::aarect::{XyRect, YzRect, XzRect};
use crate::aabb::Aabb;

//let secret_number = ;
fn random_doouble() -> f64 {
    rand::thread_rng().gen_range(1..101) as f64 / 102.0
}

fn range_random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random_doouble()
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; } else if x > max { return max; }
    return x;
}
fn clamp1(x: u32, min: u32, max: u32) -> u32 {
    if x < min { return min; } else if x > max { return max; }
    return x;
}
fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    let oc = r.ori - center;
    let a = Vec3::squared_length(&r.dic);
    let half_b = Vec3::dot(r.dic, oc);
    let c = Vec3::squared_length(&oc) - radius * radius;
    let discriminant = (half_b * half_b - a * c) as f64;
    if discriminant < 0.0 { -1.0 } else { (-half_b - discriminant.sqrt()) / (a) }
}

fn color(x: Ray,background:Vec3, world: &HittableList, dep: u32) -> Vec3 {
    if dep <= 0 { return Vec3::new(0.0, 0.0, 0.0); }
    if let Option::Some(_rec) = world.hit(x, 0.001, INFINITY as f64) {
    let mut scattered =  Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
        let mut attrnuation =Vec3::zero();
        let emitted=_rec.mat_ptr.emitted(_rec.u,_rec.v,&_rec.p);

        if !_rec.mat_ptr.scatter(&x, &_rec, &mut attrnuation, &mut scattered) {
            return emitted;
        }
        return emitted+color(scattered,background,world,dep-1)*attrnuation;




    }
    else { return background; }




    // let mut t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, x);
    // if let Option::Some(_rec) = world.hit(x, 0.001, INFINITY as f64) {
    //     let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(),0.0);
    //     let mut attenuation = Vec3::zero();
    //     if _rec.mat_ptr.scatter(&x, &_rec, &mut attenuation, &mut scattered) {
    //         return attenuation * color(scattered, world, dep - 1);
    //     }
    //
    //     return Vec3::zero();
    //     // color(Ray::new(_rec.p, _rec.p + Vec3::random_in_himisphere(_rec.normal) - _rec.p), world, dep - 1) * 0.5//apply second reflect way
    //
    //     //  (   _rec.normal+Vec3::new(1.0,1.0,1.0))*0.5
    // } else {
    //     let unit_dir: Vec3 = Vec3::unit((x.dic));
    //     t = 0.5 * ((unit_dir.y.clone() + 1.0) as f64);
    //     Vec3::new(1.0, 1.0, 1.0) * ((1.0 - t) as f64) + Vec3::new(0.5, 0.7, 1.0) * t as f64
    // }
}

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);
    //image


  //  let ratio: f64 = 16.0 / 9.0;
    let ratio: f64 = 1.0;

    // let image_width = 400 as u32;
    let image_width = 800 as u32;
    let image_heigth = (image_width as f64 / ratio) as u32;
    let sample_per_pixel = 10;//ought to be 100  可以做的更大比如500//todo
    let max_depth = 50;//an bo modifyed to lessen the time

    //world
  //let world=random_sence();

//let world=simple_light();
    let world=final_book2_scence();
   // let world=earth();
 //  let world=two_spheres();//todo
    {
        // {
        //     let mut world = HittableList {
        //         objects: vec![],
        //     };
        //     let sph1 = Sphere {
        //         p: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         normal: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         t: 0.0,
        //         center: Vec3 {
        //             x: 0.0,
        //             y: -100.5,
        //             z: -1.0,
        //         },
        //         radius: 100.0,
        //         mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),//todo
        //     };
        //     let sph2 = Sphere {
        //         p: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         normal: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         t: 0.0,
        //         center: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: -1.0,
        //         },
        //         radius: 0.5,
        //         mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
        //     };
        //     let sphleft = Sphere {
        //         p: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         normal: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         t: 0.0,
        //         center: Vec3 {
        //             x: -1.0,
        //             y: 0.0,
        //             z: -1.0,
        //         },
        //         radius: 0.5,
        //         mat_ptr: Arc::new((Dielectric::new(1.5))),
        //     };
        //     let sphleft_transparental = Sphere {
        //         p: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         normal: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         t: 0.0,
        //         center: Vec3 {
        //             x: -1.0,
        //             y: 0.0,
        //             z: -1.0,
        //         },
        //         radius: -0.4,
        //         mat_ptr: Arc::new((Dielectric::new(1.5))),
        //     };
        //     let sphright = Sphere {
        //         p: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         normal: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         t: 0.0,
        //         center: Vec3 {
        //             x: 1.0,
        //             y: 0.0,
        //             z: -1.0,
        //         },
        //         radius: 0.5,
        //         mat_ptr: Arc::new((Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0))),
        //     };
        //     let R = (PI / 4.0).cos();
        //     let materialleft = Sphere {
        //         p: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         normal: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         t: 0.0,
        //         center: Vec3 {
        //             x: -R,
        //             y: 0.0,
        //             z: -1.0,
        //         },
        //         radius: R,
        //         mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0))),//todo
        //     };
        //     let materialright = Sphere {
        //         p: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         normal: Vec3 {
        //             x: 0.0,
        //             y: 0.0,
        //             z: 0.0,
        //         },
        //         t: 0.0,
        //         center: Vec3 {
        //             x: R,
        //             y: 0.0,
        //             z: -1.0,
        //         },
        //         radius: R,
        //         mat_ptr: Arc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0))),//todo
        //     };
        //     { // world.add(
        //         //     Arc::new(materialleft)
        //         // );
        //         // world.add(
        //         //     Arc::new(materialright)
        //         // );
        //     }//two obj
        //     {
        //         world.add(
        //             Arc::new(sph1)
        //         );
        //         world.add(
        //             Arc::new(sph2)
        //         );
        //         world.add(
        //             Arc::new(sphleft)
        //         );
        //         world.add(
        //             Arc::new(sphleft_transparental)
        //         );
        //         world.add(
        //             Arc::new(sphright)
        //         );
        //     }//five
        // }
    }

//let backgroud=Vec3::new(0.7,0.8,1.0);
    let backgroud=Vec3::new(0.0,0.0,0.0);





    //Camera

    let lookfrom=Vec3::new(278.0,278.0,-800.0);//13 2 3
    let lookfrom=Vec3::new(478.0,278.0,-600.0);//13 2 3

    // let lookfrom=Vec3::new(13.0,2.0,3.0);//13 2 3

  // let lookfrom=Vec3::new(26.0,3.0,6.0);//13 2 3


    //let lookat=Vec3::new(0.0,0.0,0.0);
   //let lookat=Vec3::new(0.0,2.0,0.0);
    let lookat=Vec3::new(278.0,278.0,0.0);

    let vup=Vec3::new(0.0,1.0,0.0);
    let dist_to_focus=10.0;
    let aperture=0.1;//ought to be 2
    let cam = camera::Camera::new(lookfrom,lookat,vup,40.0,ratio,aperture,dist_to_focus,0.0,1.0);

    let view_heigth: f64 = 2.0;
    let view_width = (view_heigth * ratio) as f64;
    let focallength = 1.0;
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizon: Vec3 = Vec3::new(view_width as f64, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, view_heigth, 0.0);
    let leftcorner = origin - horizon / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focallength);

    let mut img: RgbImage = ImageBuffer::new(image_width, image_heigth);
    let bar = ProgressBar::new(1024);

    for j in 0..image_heigth {
        for i in 0..image_width
        {
            let pixel = img.get_pixel_mut(i, j);
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..sample_per_pixel {
                let u = (i as f64 + random_doouble()) / (image_width - 1) as f64;
                let v = (image_heigth as f64 - j as f64 + random_doouble()) / (image_heigth - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += color(r, backgroud,&world, max_depth);
            }


            // let aa: f32 = ((i) as f32 / (image_width-1)as f32) as f32;
            // let bb: f32 = ((image_heigth-j) as f32 / (image_heigth-1)as f32) as f32;
            // let r=Ray::new(origin,leftcorner+horizon*aa as f64+vertical*bb as f64-origin);
            // let output:Vec3=color(r, &world);
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

    img.save("output/test.png").unwrap();
    bar.finish();
}
pub fn two_spheres()->HittableList{
    let mut world = HittableList {
        objects: vec![],
    };

    let checker=Arc::new(CheckerTexture::new(Vec3::new(0.2,0.3,0.1), Vec3::new(0.9,0.9,0.9)));
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
        mat_ptr: Arc::new(Lambertian::new1(checker)),//todo
    };
    world.add(
        Arc::new(below)
    );
    let checker1=Arc::new(CheckerTexture::new(Vec3::new(0.2,0.3,0.1), Vec3::new(0.9,0.9,0.9)));

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
        mat_ptr: Arc::new(Lambertian::new1(checker1)),//todo
    };
    world.add(
        Arc::new(above)
    );
    return world;
}
fn random_sence()->HittableList{
    let mut world = HittableList {
        objects: vec![],
    };
    let checker=Arc::new(CheckerTexture::new(Vec3::new(0.2,0.3,0.1), Vec3::new(0.9,0.9,0.9)));
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
        mat_ptr: Arc::new(Lambertian::new1(checker)),//todo
    };
    world.add(
        Arc::new(ground)
    );
    for a in -11..11 {
        for b in -11..11  {
            let choose_mat=random_doouble();
            let center=Vec3::new(a as f64+0.9*random_doouble(),0.2,b as f64+0.9*random_doouble());

            if (center-Vec3::new(4.0,0.2,0.0)).length()>0.9 {
               if choose_mat<0.8 {
                   let albedo=Vec3::random()*Vec3::random();
                   let checker=Arc::new(CheckerTexture::new(Vec3::new(albedo.x,albedo.y,albedo.z), Vec3::new(albedo.x,albedo.y,albedo.z)));

                   let center2=center+Vec3::new(0.0,range_random_double(0.0,0.5),0.0);
                   let temp = MovingSphere{
                       center0: center,
                       center1: center2,
                       time0: 0.0, time1: 1.0,
                       radius: 0.2,
                       mat_ptr: Arc::new(Lambertian::new(albedo)),

                   };
                   world.add(
                       Arc::new(temp)
                   );


               }
                else if choose_mat<0.95 {
                    let albedo=Vec3::randomrange(0.5,1.0);
                    let fuzz=range_random_double(0.0,0.5);
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
                            x: center.x.clone(),
                            y: center.y.clone(),
                            z: center.z.clone(),
                        },
                        radius: 0.2,
                        mat_ptr: Arc::new((Metal::new(albedo,fuzz))),
                    };
                    world.add(
                        Arc::new(temp)
                    );

                }
                else {
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
                            x: center.x.clone(),
                            y: center.y.clone(),
                            z: center.z.clone(),
                        },
                        radius: 0.2,
                        mat_ptr: Arc::new((Dielectric::new(1.5))),
                    };


                    world.add(
                        Arc::new(temp)
                    );


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
        mat_ptr: Arc::new((Dielectric::new(1.5))),
    };
    world.add(
        Arc::new(material1)
    );



    let checker2=Arc::new(CheckerTexture::new(Vec3::new(0.4,0.2,0.1), Vec3::new(0.4,0.2,0.2)));

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
        mat_ptr: Arc::new((Lambertian::new(Vec3::new(0.4,0.2,0.1)))),
    };
    world.add(
        Arc::new(material2)
    );

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
        mat_ptr: Arc::new(Metal::new(Vec3::new(0.7,0.6,0.5),0.0)),
    };
    world.add(
        Arc::new(material3)
    );
    return world;
}
fn two_berlin_spheres()->HittableList
{
    let mut world = HittableList {
        objects: vec![],
    };

    let checker=Arc::new(NoiseTexture::new(4.0));
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
        mat_ptr: Arc::new(Lambertian::new1(checker)),//todo
    };
    world.add(
        Arc::new(below)
    );

    let checker1=Arc::new(NoiseTexture::new(4.0));
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
        mat_ptr: Arc::new(Lambertian::new1(checker1)),//todo
    };
    world.add(
        Arc::new(above)
    );


//todo
    return world;

}
fn earth()->HittableList{
    let mut world = HittableList {
        objects: vec![],
    };

    let checker=Arc::new(ImageTexture::new("earthmap.jpg"));
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
        mat_ptr: Arc::new(Lambertian::new1(checker)),//todo
    };
    world.add(
        Arc::new(below)
    );
    return world;
}

fn simple_light()->HittableList{
    let mut world = HittableList {
        objects: vec![],
    };

    let checker=Arc::new(NoiseTexture::new(4.0));
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
        mat_ptr: Arc::new(Lambertian::new1(checker)),//todo
    };
    world.add(
        Arc::new(below)
    );

    let checker1=Arc::new(NoiseTexture::new(4.0));
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
        mat_ptr: Arc::new(Lambertian::new1(checker1)),//todo
    };
    world.add(
        Arc::new(above)
    );

    let difflight=Arc::new(DiffuseLight::new(Vec3::new(4.0,4.0,4.0)));
    let difflight1 = XyRect{
        mp: Arc::new(DiffuseLight::new(Vec3::new(4.0,4.0,4.0))),
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0
    };
    world.add(
        Arc::new(difflight1)
    );
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
        mat_ptr: Arc::new(DiffuseLight::new(Vec3::new(4.0,4.0,4.0))),
    };
    world.add(
        Arc::new(light)
    );


//todo
    return world;
}

fn cornell_box()->HittableList {
    let mut world = HittableList {
        objects: vec![],
    };

    let red = YzRect {
        mp: Arc::new(((Lambertian::new(Vec3::new(0.65,0.05,0.05))))),
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0
    };
    world.add(
        Arc::new(red)
    );

    let green = YzRect {
        mp: Arc::new(((Lambertian::new(Vec3::new(0.12,0.45,0.15))))),

        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0
    };
    world.add(
        Arc::new(green)
    );
    let white1 = XzRect {
        mp: Arc::new(((Lambertian::new(Vec3::new(0.73,0.73,0.73))))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0
    };
    world.add(
        Arc::new(white1)
    );
    let white2 = XzRect {
        mp: Arc::new(((Lambertian::new(Vec3::new(0.73,0.73,0.73))))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0
    };
    world.add(
        Arc::new(white2)
    );

    let white3 = XyRect {
        mp: Arc::new(((Lambertian::new(Vec3::new(0.73,0.73,0.73))))),

        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,

    };
    world.add(
        Arc::new(white3)
    );


    let mut whitebox4 =Box1{
        box_min: Vec3::zero(),
        box_max: Vec3::zero(),
        sides: HittableList { objects: vec![] }
    };
    // whitebox4 = Box1::new( &Vec3::new(130.0, 0.0, 65.0), &Vec3::new(295.0, 165.0, 230.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    //
    // world.add(
    //     Arc::new(whitebox4)
    // );


    let mut whitebox1:Arc<dyn Hittable>=Arc::new(Box1::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(165.0, 330.0, 165.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73)))))));
    whitebox1=Arc::new(RotateY::new(whitebox1,15.0));
    whitebox1=Arc::new(Translate::new(whitebox1,Vec3::new(265.0,0.0,295.0)));
    world.add(
        whitebox1
    );



    let mut whitebox2:Arc<dyn Hittable>=Arc::new(Box1::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(165.0, 165.0, 165.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73)))))));
    whitebox2=Arc::new(RotateY::new(whitebox2,-18.0));
    whitebox2=Arc::new(Translate::new(whitebox2,Vec3::new(130.0,0.0,65.0)));
    world.add(
        whitebox2
    );

    // let whitebox5 = Box1::new( &Vec3::new(265.0, 0.0, 295.0), &Vec3::new(430.0, 330.0, 460.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    // world.add(
    //     Arc::new(whitebox5)
    // );










    let light1 = XzRect{
        mp: Arc::new(DiffuseLight::new(Vec3::new(15.0,15.0,15.0))),
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,

    };
    world.add(
        Arc::new(light1)
    );
    return world;

}

fn cornell_smoke()->HittableList {
    let mut world = HittableList {
        objects: vec![],
    };
    let red = YzRect {
        mp: Arc::new(((Lambertian::new(Vec3::new(0.65,0.05,0.05))))),
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0
    };
    world.add(
        Arc::new(red)
    );

    let green = YzRect {
        mp: Arc::new(((Lambertian::new(Vec3::new(0.12,0.45,0.15))))),

        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0
    };
    world.add(
        Arc::new(green)
    );
    let white1 = XzRect {
        mp: Arc::new(((Lambertian::new(Vec3::new(0.73,0.73,0.73))))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0
    };
    world.add(
        Arc::new(white1)
    );
    let white2 = XzRect {
        mp: Arc::new(((Lambertian::new(Vec3::new(0.73,0.73,0.73))))),

        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0
    };
    world.add(
        Arc::new(white2)
    );
    let white3 = XyRect {
        mp: Arc::new(((Lambertian::new(Vec3::new(0.73,0.73,0.73))))),

        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,

    };
    world.add(
        Arc::new(white3)
    );
    let mut whitebox4 =Box1{
        box_min: Vec3::zero(),
        box_max: Vec3::zero(),
        sides: HittableList { objects: vec![] }
    };
    // whitebox4 = Box1::new( &Vec3::new(130.0, 0.0, 65.0), &Vec3::new(295.0, 165.0, 230.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    //
    // world.add(
    //     Arc::new(whitebox4)
    // );
    let mut whitebox1:Arc<dyn Hittable>=Arc::new(Box1::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(165.0, 330.0, 165.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73)))))));
    whitebox1=Arc::new(RotateY::new(whitebox1,15.0));
    whitebox1=Arc::new(Translate::new(whitebox1,Vec3::new(265.0,0.0,295.0)));
    whitebox1=Arc::new(ConstantMedium::new(whitebox1,0.01,Vec3::new(0.0,0.0,0.0)));
    world.add(
        whitebox1
    );
    let mut whitebox2:Arc<dyn Hittable>=Arc::new(Box1::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(165.0, 165.0, 165.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73)))))));
    whitebox2=Arc::new(RotateY::new(whitebox2,-18.0));
    whitebox2=Arc::new(Translate::new(whitebox2,Vec3::new(130.0,0.0,65.0)));
    whitebox2=Arc::new(ConstantMedium::new(whitebox2,0.01,Vec3::new(1.0,1.0,1.0)));
    world.add(
        whitebox2
    );
    // let whitebox5 = Box1::new( &Vec3::new(265.0, 0.0, 295.0), &Vec3::new(430.0, 330.0, 460.0), Arc::new(((Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))));
    // world.add(
    //     Arc::new(whitebox5)
    // );
    let light1 = XzRect{
        mp: Arc::new(DiffuseLight::new(Vec3::new(7.0,7.0,7.0))),
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,

    };
    world.add(
        Arc::new(light1)
    );
    return world;
}
fn final_book2_scence()->HittableList {
    let mut world = HittableList {
        objects: vec![],
    };
    let light1 = XzRect{
        mp: Arc::new(DiffuseLight::new(Vec3::new(7.0,7.0,7.0))),
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,

    };
    world.add(
        Arc::new(light1)
    );



    let center1=Vec3::new(400.0,400.0,200.0);
    let center2=center1+Vec3::new(30.0,0.0,0.0);
    let temp = MovingSphere{
        center0: center1,
        center1: center2,
        time0: 0.0, time1: 1.0,
        radius: 50.0,
        mat_ptr: Arc::new(Lambertian::new(Vec3::new(0.7,0.3,0.1))),

    };
    world.add(
        Arc::new(temp)
    );


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
        mat_ptr: Arc::new((Metal::new(Vec3::new(0.8,0.8,0.9),1.0))),
    };
    world.add(
        Arc::new(tempmetal)
    );


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
        mat_ptr: Arc::new((Dielectric::new(1.5))),
    };


    world.add(
        Arc::new(tempdielectric)
    );

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
        mat_ptr: Arc::new((Dielectric::new(1.5))),
    };


    world.add(
        Arc::new(tempdielectric2)
    );
    let mut boudary:Arc<dyn Hittable>=Arc::new(Sphere::new(Vec3::zero(), Vec3::zero(), 0.0, Vec3::new(360.0,150.0,145.0), 70.0, Arc::new((Dielectric::new(1.5)))));
    boudary=Arc::new(ConstantMedium::new(boudary,0.2,Vec3::new(0.2,0.4,0.9)));
    world.add(
        boudary
    );
    let mut boudary2:Arc<dyn Hittable>=Arc::new(Sphere::new(Vec3::zero(), Vec3::zero(), 0.0, Vec3::new(0.0,0.0,0.0), 5000.0, Arc::new((Dielectric::new(1.5)))));
    boudary2=Arc::new(ConstantMedium::new(boudary2,0.0001,Vec3::new(1.0,1.0,1.0)));
    world.add(
        boudary2
    );
    let checker=Arc::new(ImageTexture::new("earthmap.jpg"));
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
        mat_ptr: Arc::new(Lambertian::new1(checker)),//todo
    };
    world.add(
        Arc::new(below)
    );
    let checker=Arc::new(NoiseTexture::new(0.1));
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
        radius:80.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)),//todo
    };
    world.add(
        Arc::new(pertext)
    );






    return world;
}
