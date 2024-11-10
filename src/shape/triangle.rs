pub use crate::shape::hittable::*;
pub use glam::Mat4;

#[derive(Clone)]
pub struct Triangle {
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
    normal: Vec3,
    denominator: f32,
    material: Material,
}

impl Triangle {
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3, material: Material) -> Self {
        let r1 = p2 - p1;
        let r2 = p3 - p1;
        let cross_r1_r2 = r1.cross(r2);
        let normal = cross_r1_r2.normalize();
        let denominator = normal.dot(cross_r1_r2);
        Self {
            p1,
            p2,
            p3,
            denominator,
            normal,
            material,
        }
    }

    pub fn transform(&mut self, transform_matrix: &Mat4) {
        self.p1 = transform_matrix.transform_point3(self.p1);
        self.p2 = transform_matrix.transform_point3(self.p2);
        self.p3 = transform_matrix.transform_point3(self.p3);
        let r1 = self.p2 - self.p1;
        let r2 = self.p3 - self.p1;
        let cross_r1_r2 = r1.cross(r2);
        self.normal = cross_r1_r2.normalize();
        self.denominator = self.normal.dot(cross_r1_r2);
    }
}

impl Hittable for Triangle {
    fn hits(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        let t = self.normal.dot(self.p1 - ray.origin) / self.normal.dot(ray.direction);
        if !interval.contains(&t) {
            return None;
        }

        let hit_point = ray.at(t);
        let p1_hp = self.p1 - hit_point;
        let c1 = self.normal.dot((self.p3 - hit_point).cross(p1_hp)) / self.denominator;
        let c2 = self.normal.dot(p1_hp.cross(self.p2 - hit_point)) / self.denominator;
        if c1 < 0.0 || c2 < 0.0 || c1 + c2 > 1.0 {
            return None;
        }

        Some(HitRecord::new(t, hit_point, &ray, self.normal, &self.material))
    }
}
