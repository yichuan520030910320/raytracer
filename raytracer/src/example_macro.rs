#![allow(dead_code)]
// You SHOULD remove above line in your code.

//use raytracer_codegen::make_spheres_impl;
#[allow(unused_imports)]
use crate::vec3::Vec3;

// Call the procedural macro, which will become `make_spheres` function.
//make_spheres_impl! {}

// These three structs are just written here to make it compile.
// You should `use` your own structs in this file.
// e.g. replace next two lines with
// `use crate::materials::{DiffuseLight, ConstantTexture}`
// pub struct ConstantTexture(Vec3);
// pub struct DiffuseLight(ConstantTexture);

// pub struct Sphere {
//     center: Vec3,
//     radius: f64,
//     material: DiffuseLight,
// }
// pub struct World {
//     pub height: u32,
// }
//
// impl World {
//     pub fn color(&self, _: u32, y: u32) -> u8 {
//         (y * 256 / self.height) as u8
//     }
// }
//
// pub fn example_scene() -> World {
//     let mut spheres: Vec<Box<Sphere>> = make_spheres(); // Now `spheres` stores two spheres.
//     let mut hittable_list = vec![];
//     // You can now add spheres to your own world
//     hittable_list.append(&mut spheres);
//
//     hittable_list.clear();
//     World { height: 512 }
// }
