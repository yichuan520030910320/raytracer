
mod run;
mod scence;
mod material;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rtweekend;
mod texture;
mod vec3;
mod aabb;
mod aarect;
mod camera;
mod hittable;
mod staticscence;

use crate::run::{run, runstatic};
fn main() {
    run();
    //runstatic();
}
