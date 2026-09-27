use crate::math::{random, Vec3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randfloat: [f32; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = [0.0; POINT_COUNT];
        for value in randfloat.iter_mut() {
            *value = random::random_rng(0.0, 1.0);
        }

        Self {
            randfloat,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    pub fn noise(&self, point: &Vec3) -> f32 {
        let u = point.x - f32::floor(point.x);
        let v = point.y - f32::floor(point.y);
        let w = point.z - f32::floor(point.z);

        let i = f32::floor(point.x) as i32;
        let j = f32::floor(point.y) as i32;
        let k = f32::floor(point.z) as i32;

        let mut c = [[[0.0; 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    let pi = self.perm_x[((i + di as i32) & 255) as usize];
                    let pj = self.perm_y[((j + dj as i32) & 255) as usize];
                    let pk = self.perm_z[((k + dk as i32) & 255) as usize];

                    *ck = self.randfloat[(pi ^ pj ^ pk) as usize];
                }
            }
        }

        Self::trilinear_interpolate(c, u, v, w)
    }

    fn generate_perm() -> [i32; POINT_COUNT] {
        let mut p = [0i32; POINT_COUNT];
        for (i, value) in p.iter_mut().enumerate() {
            *value = i as i32;
        }

        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut [i32; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let target = random::random_rng(0.0, (i + 1) as f32) as usize;
            p.swap(i, target);
        }
    }

    fn trilinear_interpolate(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f32 * u + (1 - i) as f32 * (1.0 - u))
                        * (j as f32 * v + (1 - j) as f32 * (1.0 - v))
                        * (k as f32 * w + (1 - k) as f32 * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }

        accum
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
