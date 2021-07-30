use crate::texture::Texture;
use crate::{random_doouble, Vec3};
use rand::Rng;

const POINT_COUNT: usize = 256;

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

        return Self {
            ranvec: raanv,
            ranfloat: rancopy,
            perm_x: perx,
            perm_y: pery,
            perm_z: perz,
        };
    }
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

    pub fn permute(arr: &mut [i32; POINT_COUNT], n: i32) {
        for i in n - 1..0 {
            let axis = rand::thread_rng().gen_range(0..i);
            let tmp = arr[i as usize];
            arr[i as usize] = arr[axis as usize];
            arr[axis as usize] = tmp;
        }
    }
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
        return Perlin::trilinear_interp(c, u, v, w);

        // let i = ((4.0 * p.x) as i32) & 255;
        // let j = ((4.0 * p.y) as i32) & 255;
        // let k = ((4.0 * p.z) as i32) & 255;
        // return self.ranfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize];
    }

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
        return accum;
    }
    pub fn turb(&self, p: Vec3, mut depth: i32) -> f64 {
        depth = 7;
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * Perlin::noise(&self, temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        return accum.abs();
    }
}

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
        return Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * ((self.scale * p.z + 10.0 * self.noise.turb(*p, 0)).sin() + 1.0);
    }
}
