pub use glam::f32::Vec3;

pub struct OrthonormalBasis {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl OrthonormalBasis {
    pub fn new(normal: &Vec3) -> Self {
        let w = normal.normalize();
        let a = if w.x.abs() > 0.9 {
            Vec3::Y
        } else {
            Vec3::X
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);

        Self {
            u,
            v,
            w
        }
    }

    pub fn transform(&self, v: &Vec3) -> Vec3 {
        v.x * self.u + v.y * self.v + self.w * v.z
    }
}