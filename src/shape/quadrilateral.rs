pub use crate::shape::hittable::*;

#[derive(Clone)]
pub struct Quadrilateral {
    pub starting_corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub normal: Vec3,
    pub d: f32,
    pub w: Vec3,
    pub material: Material,
}

impl Quadrilateral {
    pub fn new(starting_corner: Vec3, u: Vec3, v: Vec3, material: Material) -> Self {
        let n = u.cross(v);
        let normal = n.normalize();
        let d = normal.dot(starting_corner);
        let w = n / n.length_squared();

        Self {
            starting_corner,
            u,
            v,
            normal,
            d,
            w,
            material,
        }
    }

    fn get_uv(alpha: f32, beta: f32) -> Option<(f32, f32)> {
        let unit_interval = 0.0..1.0;
        if !unit_interval.contains(&alpha) || !unit_interval.contains(&beta) {
            return None;
        }

        Some((alpha, beta))
    }
}

impl Hittable for Quadrilateral {
    fn hits(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        let denominator = self.normal.dot(ray.direction);
        if denominator.abs() < 0.0001 {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin)) / denominator;
        if !interval.contains(&t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_offset = intersection - self.starting_corner;
        let alpha = self.w.dot(planar_offset.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_offset));
        let (u, v) = match Self::get_uv(alpha, beta) {
            Some((u, v)) => (u, v),
            None => return None,
        };

        let mut hit_record =
            HitRecord::new(t, intersection, &ray, self.normal, &self.material);
        hit_record.set_uv((u, v));

        Some(hit_record)
    }
}
