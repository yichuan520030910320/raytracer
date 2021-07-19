use std::f64::consts::PI;

fn degrees_to_radians(degrees:f64) ->f64 {
 degrees * PI / 180.0
}
fn fmin1(a:f64,b:f64)->f64{
 if a<b {return a; }
 return b;
}
fn fmax1(a:f64,b:f64)->f64{
 if a<b {return b; }
 return a;
}