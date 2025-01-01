pub use crate::material::*;
use crate::util;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Texture,
    pub fuzz: f32,
}

impl Scatters for Metal {
    fn scatters(&self, hit_record: &HitRecord) -> Option<Scattered> {
        let mut reflected = hit_record.ray.direction.reflect(hit_record.normal);
        reflected += self.fuzz * util::random_unit_vector();
        let scattered = Ray::with_time(
            hit_record.hit_point,
            reflected.normalize(),
            hit_record.ray.time,
        );

        if scattered.direction.dot(hit_record.normal) <= 0.0 {
            return None;
        }

        Some(Scattered {
            attenuation: self.albedo.get_color_value(
                hit_record.u,
                hit_record.v,
                hit_record.hit_point,
            ),
            pdf: None,
            scattered,
        })
    }
}
