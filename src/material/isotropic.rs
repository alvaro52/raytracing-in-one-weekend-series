pub use crate::material::*;
pub use crate::texture::*;
use crate::util;

#[derive(Clone)]
pub struct Isotropic {
    pub albedo: Texture,
}

impl Scatters for Isotropic {
    fn scatters(&self, hit_record: &HitRecord) -> Option<Scattered> {
        Some(Scattered {
            attenuation: self.albedo.get_color_value(
                hit_record.u,
                hit_record.v,
                hit_record.hit_point,
            ),
            scattered: Ray::with_time(
                hit_record.hit_point,
                util::random_unit_vector(),
                hit_record.ray.time,
            ),
            pdf: Some(
                PDF::sphere()
            ),
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, _hit_record: &HitRecord, _scattered: &Ray) -> f32 {
        1.0 / (4.0 * std::f32::consts::PI)
    }
}
