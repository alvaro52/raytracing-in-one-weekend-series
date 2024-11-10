pub mod builder;
pub mod ray;

pub use crate::camera::ray::*;
use crate::util;

pub struct Camera {
    position: Vec3,
    look_at: Vec3,
    up: Vec3,
    upper_left: Vec3,
    delta_u: Vec3,
    delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    defocus_angle: Option<f32>,
    focus_dist: f32,
    pub image_width: u32,
    pub image_height: u32,
    viewport_width: f32,
    viewport_height: f32,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let origin = if self.defocus_angle.is_some() {
            self.defocus_disk_sample()
        } else {
            self.position
        };
        let viewport_point = self.upper_left + self.delta_u * u - self.delta_v * v;

        Ray::with_time(
            origin,
            (viewport_point - origin).normalize(),
            rand::random::<f32>(),
        )
    }

    pub fn get_orthogonal_ray(&self, u: f32, v: f32) -> Ray {
        Ray::with_time(
            self.upper_left + self.delta_u * u - self.delta_v * v,
            self.look_at,
            rand::random::<f32>(),
        )
    }

    pub fn change(&mut self, position: Vec3, look_at: Vec3, up: Vec3) {
        self.position = position;
        self.look_at = (look_at - position).normalize();
        self.up = up;

        let w = -self.look_at;
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let vp_u = self.viewport_width * u;
        let vp_v = self.viewport_height * v;

        self.upper_left = position - vp_u / 2.0 + vp_v / 2.0 - (self.focus_dist * w);
        self.delta_u = vp_u / self.image_width as f32;
        self.delta_v = vp_v / self.image_height as f32;
        let defocus_radius = if let Some(defocus_angle) = self.defocus_angle {
            self.focus_dist * (defocus_angle / 2.0).to_radians().tan()
        } else {
            0.0
        };
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = util::random_in_unit_disk();
        self.position + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }
}
