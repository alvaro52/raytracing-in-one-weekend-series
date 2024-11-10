pub use crate::material::*;
pub use crate::texture::*;
use crate::util;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Texture,
}

impl Scatters for Lambertian {
    fn scatters(&self, hit_record: &HitRecord) -> Option<Scattered> {
        let scattered_direction = hit_record.normal + util::random_unit_vector();
        let mut scattered = Ray::with_time(
            hit_record.hit_point,
            scattered_direction.normalize(),
            hit_record.ray.time,
        );

        if util::near_zero(scattered.direction) {
            scattered.direction = hit_record.normal;
        }

        Some(Scattered {
            attenuation: self.albedo.get_color_value(
                hit_record.u,
                hit_record.v,
                hit_record.hit_point,
            ),
            scattered,
        })
    }
}
