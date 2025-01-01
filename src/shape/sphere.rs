use rand::Rng;
use crate::pdf::orthonormal_basis::OrthonormalBasis;
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

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f32 {
        if let Some(_) = self.hits(&Ray::new(*origin, *direction), 0.001..f32::INFINITY) {
            let distance_squared = (self.center.at(0.0) - origin).length_squared();
            let cosine = (1.0 - self.radius * self.radius / distance_squared).sqrt();
            let solid_angle = 2.0 * std::f32::consts::PI * (1.0 - cosine);

            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let direction = self.center.at(0.0) - *origin;
        let distance_squared = direction.length_squared();
        let uvw = OrthonormalBasis::new(&direction);

        uvw.transform(&Sphere::random(self.radius, distance_squared))
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
            material
        }
    }

    pub fn get_uv(direction: Vec3) -> (f32, f32) {
        let u = 0.5 + direction.x.atan2(direction.z) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - direction.y.asin() / std::f32::consts::PI;
        (u, v)
    }

    pub fn random(radius: f32, distance_squared: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        let r1 = rng.random::<f32>();
        let r2 = rng.random::<f32>();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * std::f32::consts::PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Vec3::new(x, y, z)
    }
}
