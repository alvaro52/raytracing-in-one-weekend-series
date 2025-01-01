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

        Some(Scattered {
            attenuation: self.albedo.get_color_value(
                hit_record.u,
                hit_record.v,
                hit_record.hit_point,
            ),
            scattered: Ray::with_time(
                hit_record.hit_point,
                scattered_direction.normalize(),
                hit_record.ray.time,
            ),
            pdf: Some(PDF::cosine(&hit_record.normal)),
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = hit_record.normal.dot(scattered.direction).max(0.0);
        cosine / std::f32::consts::PI
    }
}
