#![allow(clippy::all)]

mod codegen_vec;
mod readfromjson;

use crate::codegen_vec::range_random_double;
use crate::codegen_vec::random_doouble;
use crate::codegen_vec::Vec3;
use proc_macro2::TokenStream;
use quote::quote;
use rand::Rng;
use std::sync::Arc;

#[derive(Clone)]
struct Cotenent {
    bounding: Vec3,
    code: TokenStream,
}


fn bvh_build(contenent: &mut Vec<Cotenent>) -> TokenStream {
    let span = contenent.len();
    let mut objects = contenent.clone();
    let axis = rand::thread_rng().gen_range(0..3);
    if span == 1 {
        let left = objects.remove(0);
        let temp_code = left.code;
        quote! {   #temp_code}

    }

    else{
        objects.sort_by(|a, b| {
            let x = a.bounding.get(axis);
            let y = b.bounding.get(axis);
            x.partial_cmp(&y).unwrap()
        });
        let mid = span / 2;
        let mut nmd = contenent.clone();
        let (object0, object1) = nmd.split_at_mut(mid as usize);
        let left = bvh_build(&mut object0.to_vec());
        let right = bvh_build(&mut object1.to_vec());
        quote! { Arc::new(StaticBvhNode::new(#left,#right,0.0,1.0))}

    }

}

#[proc_macro]
pub fn random_scene_static_bvh(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut objects = vec![];
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_doouble();
            println!("aaa  {}",choose_mat);
            let center = Vec3::new(
                a as f64 + 0.9 * random_doouble(),
                0.2,
                b as f64 + 0.9 * random_doouble(),
            );
            let r1 = center.x;
            let r2 = center.y;
            let r3 = center.z;
            if ((center - Vec3::new(4.0, 0.2, 0.0)).length()) > 1.19 {

                println!("got in  {}",choose_mat);
                if choose_mat < 0.6 {
                    let albedo = Vec3::random() * Vec3::random();

                    let x1 = albedo.x;
                    let x2 = albedo.y;
                    let x3 = albedo.z;
                    let fuzz = range_random_double(0.0, 0.5);
                    objects.push(Cotenent {
                        bounding: center - Vec3::new(0.2, 0.2, 0.2),
                        code: (quote! {(
                            Arc::new(
StaticSphere {
                     p: Vec3::zero(),
                        normal:Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        t: 0.0,
                        center: Vec3{x:#r1,
                            y: #r2,
                            z:#r3,} ,
                        radius: 0.2,
                        mat_ptr:  StaticLambertian::<StaticBaseColor>::new(Vec3::new(#x1, #x2, #x3)),
                    }
                            )
                        )}),
                    });
                }
                else if choose_mat<0.850{
                    print!("***");
                    let albedo = Vec3::randomrange(0.5, 1.0);
                    let x1 = albedo.x;
                    let x2 = albedo.y;
                    let x3 = albedo.z;
                    let fuzz = range_random_double(0.0, 0.5);
                    objects.push(Cotenent {
                        bounding: center - Vec3::new(0.2, 0.2, 0.2),
                        code: quote! {(
                            Arc::new(
StaticSphere {
                        p: Vec3::zero(),
                        normal:Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        t: 0.0,
                        center: Vec3{x:#r1,
                            y: #r2,
                            z:#r3,} ,
                        radius: 0.2,
                        mat_ptr: StaticMetal::new(Vec3::new(
                                        #x1,#x2,#x3
                                    ), #fuzz)
                    }
                            )
                        )},
                    });
                }
                else {


//                     objects.push(Cotenent {
//                         bounding: Vec3::new(0.3,0.3,0.3) - Vec3::new(0.2, 0.2, 0.2),
//                         code: (quote! {(
//                             Arc::new(
// StaticMovingSphere {
//                        center0: Vec3::new(0.1,0.1,0.1),
//                         center1:Vec3::new(0.2,0.2,0.2),
//                         time0:0.0,//todo
//                         time1: 1.0,
//                         radius: 0.2,
//                         mat_ptr: StaticLambertian::<StaticBaseColor>::new(Vec3::new(
//                                        0.99,0.0,0.0
//                                     )),
//                     }
//                             )
//                         )}),
//                     });
                    objects.push(Cotenent {
                        bounding: center - Vec3::new(0.2, 0.2, 0.2),
                        code: quote! {(
                            Arc::new(
StaticSphere {
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
                            x: #r1,
                            y: #r2,
                            z: #r3,
                        },
                        radius: 0.2,
                        mat_ptr: StaticDielectric::new(1.5),
                    }
                            )
                        )},
                    });
                }
            }
        }
    }


    objects.push(Cotenent {
        bounding: Vec3::new(0.3,0.3,0.3) - Vec3::new(0.2, 0.2, 0.2),
        code: (quote! {(
                            Arc::new(
StaticMovingSphere {
                       center0: Vec3::new(0.1,0.1,0.1),
                        center1:Vec3::new(0.2,0.2,0.2),
                        time0:0.0,//todo
                        time1: 1.0,
                        radius: 0.2,
                        mat_ptr: StaticLambertian::<StaticBaseColor>::new(Vec3::new(
                                       0.99,0.0,0.0
                                    )),
                    }
                            )
                        )}),
    });
println!("&&&&");
    let allnode = bvh_build(&mut objects);
    let result = proc_macro::TokenStream::from(quote! {
         fn add_bvh_static()->Arc<dyn StaticHittable>{
            let a=Vec3::new(0.0,0.0,0.0);
            let b=Vec3{
                x:0.0,y:0.0,z:0.0,
            };
                #allnode
            }
    });
    return result.into();
}
//
//
//

//
//
// /// You can replace this `Sphere` with your own version, e.g. the one
// /// you wrote in raytracer, if you want.
// #[derive(Copy, Clone, Debug)]
// struct Sphere {
//     pub pos: Vec3,
//     pub size: f64,
//     pub color: Vec3,
// }
//
//
// /// This function generates code for one Sphere.
// fn sphere_code(sphere: &Sphere) -> TokenStream {
//     let Vec3(x, y, z) = sphere.pos;
//     let Vec3(r, g, b) = sphere.color;
//     let size = sphere.size * 0.9;
//
//     // Create a code snippet of `Sphere`.
//     // Note that the `Sphere` in `quote` is the one in your ray tracer,
//     // not the one you defined in this module.
//     quote! {
//         Box::new(Sphere {
//             center: Vec3::new(#x, #y, #z),
//             radius: #size,
//             material: DiffuseLight(
//                 ConstantTexture(
//                     Vec3::new(#r, #g, #b)
//                 )
//             )
//         })
//     }
// }
//
// /// This function generates the final `make_spheres` function.
// #[proc_macro]
// pub fn make_spheres_impl(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let spheres = vec![
//         &Sphere {
//             pos: Vec3(0.0, 0.0, 0.0),
//             size: 1.0,
//             color: Vec3(1.0, 1.0, 1.0),
//         },
//         &Sphere {
//             pos: Vec3(0.0, 0.0, 0.0),
//             size: 1.0,
//             color: Vec3(1.0, 1.0, 1.0),
//         },
//     ];
//
//     let mut tokens = vec![];
//
//     for sphere in spheres {
//         let sc = sphere_code(sphere);
//         tokens.push(quote! {
//             spheres.push(#sc);
//         });
//     }
//
//     let x_final: TokenStream = tokens.into_iter().collect();
//
//     let result = proc_macro::TokenStream::from(quote! {
//         fn make_spheres() -> Vec<Box<Sphere>> {
//             let mut spheres = vec![];
//             #x_final
//             spheres
//         }
//     });
//
//    // uncomment this statement if you want to inspect result
//     println!("{}", result);
//
//     result
// }
