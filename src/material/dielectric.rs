pub use crate::material::*;

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatters for Dielectric {
    fn scatters(&self, hit_record: &HitRecord) -> Option<Scattered> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = (-hit_record.ray.direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rand::random::<f32>()
        {
            hit_record.ray.direction.reflect(hit_record.normal)
        } else {
            hit_record
                .ray
                .direction
                .refract(hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::with_time(hit_record.hit_point, direction, hit_record.ray.time);

        Some(Scattered {
            attenuation: Vec3::ONE,
            pdf: None,
            scattered,
        })
    }
}
