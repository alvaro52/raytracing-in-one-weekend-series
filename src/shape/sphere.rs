pub use crate::shape::hittable::*;

#[derive(Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f32,
    material: Material,
}

impl Hittable for Sphere {
    fn hits(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = ray.origin - current_center;
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = half_b * half_b - c;
        if delta < 0.0 {
            return None;
        }

        let sqrt_delta = delta.sqrt();
        let mut t = -half_b - sqrt_delta;
        if !interval.contains(&t) {
            t = -half_b + sqrt_delta;
            if !interval.contains(&t) {
                return None;
            }
        }

        let hit_point = ray.at(t);
        let outward_normal = (hit_point - current_center) / self.radius;
        let mut hit_record =
            HitRecord::new(t, hit_point, ray, outward_normal, &self.material);
        hit_record.set_uv(Sphere::get_uv(outward_normal));

        Some(hit_record)
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center: Ray::new(center, Vec3::ZERO),
            radius,
            material,
        }
    }

    pub fn moving(center1: Vec3, center2: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1),
            radius,
            material,
        }
    }

    pub fn get_uv(direction: Vec3) -> (f32, f32) {
        let u = 0.5 + direction.x.atan2(direction.z) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - direction.y.asin() / std::f32::consts::PI;
        (u, v)
    }
}
