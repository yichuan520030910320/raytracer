use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use crate::testbench::fibonacci;
// use mycrate::fibonacci;
include!("../src/testbench.rs");
// include!("testbench.rs");

// mod aabb;
// mod aarect;
// mod camera;
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
mod testbench;
// use crate::testbench::fibonacci;
// use crate::vec3::random_doouble;

// fn fibonacci(n: u64) -> u64 {
//      match n {
//          0 => 1,
//          1 => 1,
//          n => fibonacci(n-1) + fibonacci(n-2),
//      }
//  }

// fn fibonacci(n: u64) -> u64 {
//     let mut a = 0;
//     let mut b = 1;
//
//     match n {
//         0 => b,
//         _ => {
//             for _ in 0..n {
//                 let c = a + b;
//                 a = b;
//                 b = c;
//             }
//             b
//         }
//     }
// }

pub fn criterion_benchmark(c: &mut Criterion) {
    match project_root::get_project_root() {
        Ok(p) => println!("Current project root is {:?}", p),
        Err(e) => println!("Error obtaining project root {:?}", e),
    };
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
