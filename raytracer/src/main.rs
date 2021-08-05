mod aabb;
mod aarect;
mod camera;
mod example_macro;
mod hittable;
mod material;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rtweekend;
mod run;
mod scence;
mod staticscence;
mod texture;
mod vec3;

#[allow(unused_imports)]
use crate::run::{run, runstatic};

fn main() {
    let num = 1;
    match num {
        1 => run(),
        2 => runstatic(),
        _ => {
            println!("please rechoose the type");
        }
    }
}
