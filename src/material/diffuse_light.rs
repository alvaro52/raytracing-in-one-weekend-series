pub use crate::material::*;

#[derive(Clone)]
pub struct DiffuseLight {
    pub emission: Texture,
}

impl Scatters for DiffuseLight {
    fn emitted(&self, hit_record: &HitRecord) -> Vec3 {
        self.emission
            .get_color_value(hit_record.u, hit_record.v, hit_record.hit_point)
    }
}
