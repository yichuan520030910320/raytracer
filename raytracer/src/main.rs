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
    run();
    //runstatic();
}
