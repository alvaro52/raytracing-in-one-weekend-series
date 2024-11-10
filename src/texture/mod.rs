pub mod checkers;
pub mod image_tex;
pub mod perlin;
pub mod solid_color;

pub use glam::f32::Vec3;

#[derive(Clone)]
pub enum Texture {
    SolidColor(solid_color::SolidColor),
    Checkers(checkers::Checkers),
    ImageTex(image_tex::ImageTex),
    Perlin(perlin::Perlin),
}

pub trait ColorValue {
    fn get_color_value(&self, u: f32, v: f32, hit_point: Vec3) -> Vec3;
}

impl ColorValue for Texture {
    fn get_color_value(&self, u: f32, v: f32, hit_point: Vec3) -> Vec3 {
        match self {
            Texture::SolidColor(solid_color) => solid_color.get_color_value(u, v, hit_point),
            Texture::ImageTex(image_tex) => image_tex.get_color_value(u, v, hit_point),
            Texture::Checkers(checkers) => checkers.get_color_value(u, v, hit_point),
            Texture::Perlin(perlin) => perlin.get_color_value(u, v, hit_point),
        }
    }
}

impl Texture {
    pub fn solid_color(albedo: Vec3) -> Self {
        Self::SolidColor(solid_color::SolidColor { albedo })
    }

    pub fn checkers(even: Texture, odd: Texture, inverse_scale: f32) -> Self {
        Self::Checkers(checkers::Checkers {
            even: Box::new(even),
            odd: Box::new(odd),
            inverse_scale: 1.0 / inverse_scale,
        })
    }

    pub fn checkers_from_vec3(even: Vec3, odd: Vec3, inverse_scale: f32) -> Self {
        Self::Checkers(checkers::Checkers {
            even: Box::new(Self::solid_color(even)),
            odd: Box::new(Self::solid_color(odd)),
            inverse_scale: 1.0 / inverse_scale,
        })
    }

    pub fn image_tex(path: &str) -> Self {
        Self::ImageTex(image_tex::ImageTex::new(path))
    }

    pub fn perlin(scale: f32) -> Self {
        Self::Perlin(perlin::Perlin::new(scale))
    }
}
