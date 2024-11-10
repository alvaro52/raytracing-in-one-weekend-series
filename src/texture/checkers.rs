pub use crate::texture::*;

#[derive(Clone)]
pub struct Checkers {
    pub even: Box<Texture>,
    pub odd: Box<Texture>,
    pub inverse_scale: f32,
}

impl ColorValue for Checkers {
    fn get_color_value(&self, u: f32, v: f32, hit_point: Vec3) -> Vec3 {
        let x_int = (self.inverse_scale * hit_point.x).floor() as i32;
        let y_int = (self.inverse_scale * hit_point.y).floor() as i32;
        let z_int = (self.inverse_scale * hit_point.z).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;
        if is_even {
            self.even.get_color_value(u, v, hit_point)
        } else {
            self.odd.get_color_value(u, v, hit_point)
        }
    }
}
