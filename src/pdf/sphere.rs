use crate::pdf::*;
use crate::util;

pub struct Sphere {
}

impl ProbabilityDensityFunction for Sphere {
    fn value(&self, _direction: &Vec3) -> f32 {
        1.0 / (4.0 * std::f32::consts::PI)
    }

    fn generate(&self) -> Vec3 {
        util::random_unit_vector()
    }
}
