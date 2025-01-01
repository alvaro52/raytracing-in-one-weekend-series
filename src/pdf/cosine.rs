use crate::pdf::*;
use crate::pdf::orthonormal_basis::*;
use crate::util;

pub struct Cosine {
    onb: OrthonormalBasis
}

impl Cosine {
    pub fn new(v: &Vec3) -> Self {
        Self {
            onb: OrthonormalBasis::new(v)
        }
    }
}

impl ProbabilityDensityFunction for Cosine {
    fn value(&self, direction: &Vec3) -> f32 {
        let cosine_theta = direction.dot(self.onb.w);
        (cosine_theta / std::f32::consts::PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.onb.transform(&util::random_cosine_direction())
    }
}
