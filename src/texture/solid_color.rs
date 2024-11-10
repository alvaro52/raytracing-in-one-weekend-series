pub use crate::texture::*;

#[derive(Clone)]
pub struct SolidColor {
    pub albedo: Vec3,
}

impl ColorValue for SolidColor {
    fn get_color_value(&self, _: f32, _: f32, _: Vec3) -> Vec3 {
        self.albedo
    }
}
