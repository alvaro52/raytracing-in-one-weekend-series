pub use crate::texture::*;
use crate::util;
use rand::prelude::SliceRandom;

#[derive(Clone)]
pub struct Perlin {
    scale: f32,
    random_vecs: Vec<Vec3>,
    permutation_x: Vec<i32>,
    permutation_y: Vec<i32>,
    permutation_z: Vec<i32>,
}

impl Perlin {
    pub fn new(scale: f32) -> Self {
        let random_vecs = (0..256)
            .map(|_| util::random_vec3_in_range(-1.0, 1.0).normalize())
            .collect();
        let mut permutation_x: Vec<i32> = (0..256).collect();
        let mut permutation_y: Vec<i32> = (0..256).collect();
        let mut permutation_z: Vec<i32> = (0..256).collect();

        let mut rng = rand::thread_rng();
        permutation_x.shuffle(&mut rng);
        permutation_y.shuffle(&mut rng);
        permutation_z.shuffle(&mut rng);

        Self {
            scale,
            random_vecs,
            permutation_x,
            permutation_y,
            permutation_z,
        }
    }

    pub fn get_noise(&self, hit_point: Vec3) -> f32 {
        let u = hit_point.x - hit_point.x.floor();
        let v = hit_point.y - hit_point.y.floor();
        let w = hit_point.z - hit_point.z.floor();

        let i = hit_point.x.floor() as i32;
        let j = hit_point.y.floor() as i32;
        let k = hit_point.z.floor() as i32;
        let mut c = [[[Vec3::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = (self.permutation_x[((i + di) & 255) as usize]
                        ^ self.permutation_y[((j + dj) & 255) as usize]
                        ^ self.permutation_z[((k + dk) & 255) as usize])
                        as usize;
                    c[di as usize][dj as usize][dk as usize] = self.random_vecs[index];
                }
            }
        }

        Self::perlin_interpolation(c, u, v, w)
    }

    fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accumulated = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accumulated += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                        * c[i][j][k].dot(weight);
                }
            }
        }

        accumulated
    }

    fn turbulence(&self, hit_point: Vec3, depth: i32) -> f32 {
        let mut accumulated = 0.0;
        let mut temp_hit_point = hit_point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accumulated += weight * self.get_noise(temp_hit_point);
            weight *= 0.5;
            temp_hit_point *= 2.0;
        }

        accumulated.abs()
    }
}

impl ColorValue for Perlin {
    fn get_color_value(&self, _: f32, _: f32, hit_point: Vec3) -> Vec3 {
        Vec3::splat(0.5)
            * ((self.scale * hit_point.z + 10.0 * self.turbulence(hit_point, 7)).sin() + 1.0)
    }
}
