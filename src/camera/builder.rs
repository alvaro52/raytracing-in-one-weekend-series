use crate::camera::*;

pub struct CameraBuilder {
    position: Vec3,
    look_at: Vec3,
    up: Vec3,
    image_height: u32,
    viewport_height: f32,
    aspect_ratio: f32,
    vfov: f32,
    focus_dist: f32,
    defocus_angle: Option<f32>,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder {
            position: Vec3::ZERO,
            look_at: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            image_height: 720,
            viewport_height: 2.0,
            aspect_ratio: 16.0 / 9.0,
            vfov: 90.0,
            focus_dist: 1.0,
            defocus_angle: None,
        }
    }
}

impl CameraBuilder {
    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn looking_at(mut self, look_at: Vec3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn up(mut self, up: Vec3) -> Self {
        self.up = up;
        self
    }

    pub fn with_image_height(mut self, image_height: u32) -> Self {
        self.image_height = image_height;
        self
    }

    pub fn with_viewport_height(mut self, viewport_height: f32) -> Self {
        self.viewport_height = viewport_height;
        self
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_vfov(mut self, vfov: f32) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn with_focus_dist(mut self, focal_length: f32) -> Self {
        self.focus_dist = focal_length;
        self
    }

    pub fn with_defocus_angle(mut self, defocus_angle: f32) -> Self {
        self.defocus_angle = Some(defocus_angle);
        self
    }

    pub fn build(self) -> Camera {
        let viewport_height =
            self.viewport_height * (self.vfov.to_radians() / 2.0).tan() * self.focus_dist;
        let viewport_width = self.aspect_ratio * viewport_height;
        let image_width = (self.image_height as f32 * self.aspect_ratio) as u32;
        let look_at = (self.look_at - self.position).normalize();

        let w = -look_at;
        let u = self.up.cross(w).normalize();
        let v = w.cross(u);

        let vp_u = viewport_width * u;
        let vp_v = viewport_height * v;

        let upper_left = self.position - vp_u / 2.0 + vp_v / 2.0 - (self.focus_dist * w);
        let delta_u = vp_u / image_width as f32;
        let delta_v = vp_v / self.image_height as f32;

        let defocus_radius = if let Some(defocus_angle) = self.defocus_angle {
            self.focus_dist * (defocus_angle / 2.0).to_radians().tan()
        } else {
            0.0
        };
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            position: self.position,
            look_at,
            up: self.up,
            upper_left,
            delta_u,
            delta_v,
            image_width,
            image_height: self.image_height,
            viewport_width,
            viewport_height,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle: self.defocus_angle,
            focus_dist: self.focus_dist,
        }
    }
}
