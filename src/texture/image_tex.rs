pub use crate::texture::*;
use image::ImageReader;
pub use image::Rgb32FImage;

#[derive(Clone)]
pub struct ImageTex {
    pub image: &'static Rgb32FImage,
}

impl ImageTex {
    pub fn new(path: &str) -> Self {
        let image = match ImageReader::open(path) {
            Ok(reader) => Box::leak(Box::new(
                reader
                    .decode()
                    .expect("Error: could not decode the texture file.")
                    .into_rgb32f(),
            )),
            Err(e) => {
                eprintln!("Error: could not open the texture file {}.", path);
                panic!("Error loading image: {}", e)
            }
        };

        Self { image }
    }
}

impl ColorValue for ImageTex {
    fn get_color_value(&self, u: f32, v: f32, _p: Vec3) -> Vec3 {
        let x = (u * self.image.width() as f32) as u32 % self.image.width();
        let y = (v * self.image.height() as f32) as u32 % self.image.height();
        let pixel = self.image.get_pixel(x, y);
        Vec3::from_array(pixel.0)
    }
}
