use image::ImageReader;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Div;

use crate::camera::builder::CameraBuilder;
use crate::camera::*;
use crate::pdf::{ProbabilityDensityFunction, PDF};
use crate::shape::*;

pub struct Scene {
    pub camera: Camera,
    pub world: Vec<Shape>,
    pub light: Option<Shape>,
    pub samples: u32,
    pub max_depth: u32,
    pub background_color: Vec3,
    pub background_texture: Option<image::RgbImage>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            camera: CameraBuilder::default().build(),
            world: Vec::new(),
            light: None,
            samples: 100,
            max_depth: 50,
            background_color: Vec3::new(0.5, 0.7, 1.0),
            background_texture: None,
        }
    }

    pub fn render(&self) -> Vec<u8> {
        (0..self.camera.image_height)
            .cartesian_product(0..self.camera.image_width)
            .collect::<Vec<(u32, u32)>>()
            .into_par_iter()
            .progress_count(self.camera.image_height as u64 * self.camera.image_width as u64)
            .map(|(y, x)| {
                let mut rng = rand::thread_rng();
                let sqrt_samples = self.samples as f32;
                let sqrt_samples = sqrt_samples.sqrt().ceil() as u32;
                let sqrt_samples_recip = (sqrt_samples as f32).recip();
                (0..sqrt_samples)
                    .cartesian_product(0..sqrt_samples)
                    .map(|(x_jitter, y_jitter)| {
                        let u = x as f32 + (x_jitter as f32 + rng.random::<f32>()) * sqrt_samples_recip;
                        let v = y as f32 + (y_jitter as f32 + rng.random::<f32>()) * sqrt_samples_recip;
                        let ray = self.camera.get_ray(u, v);
                        self.ray_color(ray, self.max_depth)
                    })
                    // .map(|vec| if vec.is_nan() { Vec3::ZERO } else { vec })
                    .sum::<Vec3>()
                    .div(self.samples as f32)
                    .map(|c| c.sqrt())
                    .clamp(Vec3::ZERO, Vec3::ONE)
                    .to_array()
                    .map(|c| (c * 255.0) as u8)
            })
            .flatten()
            .collect()
    }

    fn ray_color(&self, ray: Ray, depth: u32) -> Vec3 {
        if depth == 0 {
            return Vec3::ZERO;
        }

        match self.world.hits(&ray, 0.001..f32::INFINITY) {
            None => self.get_background(ray.direction),
            Some(hit_record) => hit_record.material.scatters(&hit_record).map_or_else(
                || hit_record.material.emitted(&hit_record),
                |mut scattered| {
                    match self.light.as_ref() {
                        None => scattered.attenuation * self.ray_color(scattered.scattered, depth - 1),
                        Some(_) => self.handle_light_pdf(&ray, &hit_record, &mut scattered, depth),
                    }
                },
            ),
        }
    }

    fn handle_light_pdf(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        scattered: &mut Scattered,
        depth: u32
    ) -> Vec3 {
        let light = self.light.as_ref().unwrap();

        let light_pdf = PDF::hittable(
            light, &hit_record.hit_point
        );

        if let Some(ref scattered_pdf) = scattered.pdf {
            let pdf = PDF::mixture(&light_pdf, &scattered_pdf);

            let scattering_pdf =
                hit_record.material.scattering_pdf(&ray, &hit_record, &scattered.scattered);
            let pdf_value = pdf.value(&scattered.scattered.direction);

            (scattered.attenuation *
                self.ray_color(scattered.scattered, depth - 1) *
                scattering_pdf) /
                pdf_value
        } else {
            scattered.attenuation * self.ray_color(scattered.scattered, depth - 1)
        }
    }

    fn get_background(&self, ray_direction: Vec3) -> Vec3 {
        match &self.background_texture {
            None => self.background_color,
            Some(texture) => {
                let (u, v) = sphere::Sphere::get_uv(ray_direction);
                let x = (u * texture.width() as f32) as u32 % texture.width();
                let y = (v * texture.height() as f32) as u32 % texture.height();
                let pixel = texture.get_pixel(x, y);
                Vec3::new(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                )
            }
        }
    }

    pub fn set_background_texture(&mut self, path: &str) {
        let texture = ImageReader::open(path)
            .unwrap_or_else(|_| panic!("Failed to open background texture: {}", path))
            .decode()
            .unwrap_or_else(|_| panic!("Failed to decode background texture: {}", path))
            .into_rgb8();

        self.background_texture = Some(texture);
    }
}
