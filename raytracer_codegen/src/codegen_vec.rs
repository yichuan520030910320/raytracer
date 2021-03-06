use proc_macro2::{Ident, Span, TokenStream};
use rand::Rng;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use syn::__private::TokenStreamExt;

//let secret_number = ;
pub(crate) fn random_doouble() -> f64 {
    rand::thread_rng().gen_range(1..101) as f64 / 102.0
}

pub(crate) fn range_random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random_doouble()
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl quote::ToTokens for Vec3 {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(Ident::new("Vec3", Span::call_site()));
    }
}

#[warn(unused_parens)]
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    #[allow(dead_code)]
    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
    #[allow(dead_code)]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    #[allow(dead_code)]
    pub fn length(&self) -> f64 {
        { (self.x * self.x + self.y * self.y + self.z * self.z) as f64 }.sqrt()
    }
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn reflect(v: Self, n: Self) -> Self {
        v - n * (2.0_f64 * Vec3::dot(v, n))
    }
    #[allow(dead_code)]
    pub fn refract(uv: Self, n: Self, etai_over_etat: f64) -> Self {
        let costhta = Vec3::dot(-uv, n);
        let r_out_perp = (uv + n * costhta) * etai_over_etat;
        let r_out_parallel = n * (-(1.0 - Vec3::squared_length(&r_out_perp)).abs().sqrt());
        r_out_perp + r_out_parallel
    }
    #[allow(dead_code)]
    pub fn cross(u: Self, v: Self) -> Self {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }
    #[allow(dead_code)]
    pub fn sameside(a: Vec3, b: Vec3, c: Vec3, p: Vec3) -> bool {
        let ab = b - a;
        let ac = c - a;
        let ap = p - a;

        let v1 = Vec3::cross(ab, ac);
        let v2 = Vec3::cross(ab, ap);
        Vec3::dot(v1, v2) >= 0.0
    }
    #[allow(clippy::needless_return)]
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
