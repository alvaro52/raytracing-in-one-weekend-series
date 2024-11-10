pub use crate::shape::hittable::*;

#[derive(Clone)]
pub struct Plane {
    pub center: Vec3,
    pub normal: Vec3,
    pub radius: Option<f32>,
    pub material: Material,
}

impl Hittable for Plane {
    fn hits(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        let t = (self.center - ray.origin).dot(self.normal) / ray.direction.dot(self.normal);
        if !interval.contains(&t) {
            return None;
        }

        let hit_point = ray.at(t);
        if let Some(radius) = self.radius {
            if (hit_point - self.center).length() > radius {
                return None;
            }
        }

        Some(HitRecord::new(t, hit_point, &ray, self.normal, &self.material))
    }
}
