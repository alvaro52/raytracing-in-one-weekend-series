use crate::pdf::*;
use crate::pdf::orthonormal_basis::*;
use crate::shape;
use crate::shape::Hittable;

pub struct HittablePDF<'a> {
    shape: &'a shape::Shape,
    origin: Vec3,
}

impl<'a> HittablePDF<'a> {
    pub fn new(shape: &'a shape::Shape, origin: &Vec3) -> Self {
        Self {
            shape,
            origin: *origin,
        }
    }
}

impl<'a> ProbabilityDensityFunction for HittablePDF<'a> {
    fn value(&self, direction: &Vec3) -> f32 {
        self.shape.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.shape.random(&self.origin)
    }
}
