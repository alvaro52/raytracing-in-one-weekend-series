pub use crate::material::*;

#[derive(Clone)]
pub struct DiffuseLight {
    pub emission: Texture,
}

impl Scatters for DiffuseLight {
    fn emitted(&self, hit_record: &HitRecord) -> Vec3 {
        if !hit_record.front_face {
            return Vec3::ZERO;
        }

        self.emission
            .get_color_value(hit_record.u, hit_record.v, hit_record.hit_point)
    }
}
