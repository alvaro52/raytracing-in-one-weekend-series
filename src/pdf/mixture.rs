use rand::Rng;
use crate::pdf::*;
use crate::pdf::orthonormal_basis::*;

pub struct Mixture<'a> {
    pdf1: &'a PDF<'a>,
    pdf2: &'a PDF<'a>,
}

impl<'a> Mixture<'a> {
    pub fn new(pdf1: &'a PDF, pdf2:  &'a PDF) -> Self {
        Self {
            pdf1,
            pdf2,
        }
    }
}

impl<'a> ProbabilityDensityFunction for Mixture<'a> {
    fn value(&self, direction: &Vec3) -> f32 {
        0.5 * self.pdf1.value(direction) + 0.5 * self.pdf2.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if rand::thread_rng().random::<f32>() < 0.5 {
            self.pdf1.generate()
        } else {
            self.pdf2.generate()
        }
    }
}
