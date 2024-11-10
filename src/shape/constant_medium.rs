pub use crate::shape::hittable::*;
use crate::shape::mesh::Mesh;

#[derive(Clone)]
pub struct ConstantMedium<T: Hittable> {
    boundary: T,
    phase_function: Material,
    neg_inv_density: f32,
}
pub type SmokeCube = ConstantMedium<Mesh>;

impl<T: Hittable> ConstantMedium<T> {
    pub fn new(boundary: T, density: f32, albedo: Vec3) -> Self {
        Self {
            boundary,
            phase_function: Material::isotropic_from_vec3(albedo),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl<T: Hittable + Clone> Hittable for ConstantMedium<T> {
    fn hits(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        if let Some(mut hit_record_1) = self.boundary.hits(ray, -f32::INFINITY..f32::INFINITY) {
            if let Some(mut hit_record_2) = self
                .boundary
                .hits(ray, hit_record_1.t + 0.0001..f32::INFINITY)
            {
                if hit_record_1.t < interval.start {
                    hit_record_1.t = interval.start;
                }

                if hit_record_2.t > interval.end {
                    hit_record_2.t = interval.end;
                }

                if hit_record_1.t >= hit_record_2.t {
                    return None;
                }

                if hit_record_1.t < 0.0 {
                    hit_record_1.t = 0.0;
                }

                let distance_inside_boundary = hit_record_2.t - hit_record_1.t;
                let hit_distance = self.neg_inv_density * rand::random::<f32>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = hit_record_1.t + hit_distance;
                let hit_point = ray.at(t);
                let normal = Vec3::new(1.0, 0.0, 0.0);
                let front_face = true;

                return Some(HitRecord {
                    t,
                    hit_point,
                    normal,
                    ray: *ray,
                    front_face,
                    material: &self.phase_function,
                    u: 0.0,
                    v: 0.0,
                });
            }
        }

        None
    }
}
