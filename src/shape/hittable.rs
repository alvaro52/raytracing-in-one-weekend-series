pub use crate::camera::ray::*;
pub use crate::material::*;
pub use std::ops::Range;

pub struct HitRecord<'a> {
    pub t: f32,
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub ray: Ray,
    pub front_face: bool,
    pub u: f32,
    pub v: f32,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f32,
        hit_point: Vec3,
        ray: &Ray,
        outward_normal: Vec3,
        material: &'a Material,
    ) -> Self {
        let front_face = outward_normal.dot(ray.direction) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            t,
            hit_point,
            normal,
            ray: *ray,
            front_face,
            material,
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn set_uv(&mut self, (u, v): (f32, f32)) {
        self.u = u;
        self.v = v;
    }
}

pub trait Hittable {
    fn hits(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord>;
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hits(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        let (_, hit_record) = self.iter()
            .fold((interval.end, None), |(closest_t, closest_hit), shape| {
                if let Some(hit) = shape.hits(ray, interval.start..closest_t) {
                    (hit.t, Some(hit))
                } else {
                    (closest_t, closest_hit)
                }
        });

        hit_record
    }
}
