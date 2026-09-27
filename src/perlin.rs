use crate::math::{random, vec3, Vec3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randvec = [Vec3::ZEROS; POINT_COUNT];
        for value in randvec.iter_mut() {
            *value = random::random_vec3(-1.0, 1.0);
        }

        Self {
            randvec,
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

        let mut c = [[[Vec3::ZEROS; 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, ck) in cj.iter_mut().enumerate() {
                    let pi = self.perm_x[((i + di as i32) & 255) as usize];
                    let pj = self.perm_y[((j + dj as i32) & 255) as usize];
                    let pk = self.perm_z[((k + dk as i32) & 255) as usize];

                    *ck = self.randvec[(pi ^ pj ^ pk) as usize];
                }
            }
        }

        Self::perlin_interpolate(c, u, v, w)
    }

    pub fn turbulence(&self, point: &Vec3, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        return accum.abs();
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

    fn perlin_interpolate(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = vec3(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                        * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                        * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
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
