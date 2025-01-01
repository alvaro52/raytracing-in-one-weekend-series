pub mod orthonormal_basis;
pub mod sphere;
pub mod cosine;
pub mod hittable;
pub mod mixture;

pub use glam::f32::Vec3;
use crate::shape;

pub trait ProbabilityDensityFunction {
    fn value(&self, direction: &Vec3) -> f32;
    fn generate(&self) -> Vec3;
}

pub enum PDF<'a> {
    Cosine(cosine::Cosine),
    Sphere(sphere::Sphere),
    Mixture(mixture::Mixture<'a>),
    Hittable(hittable::HittablePDF<'a>),
}

impl<'a> PDF<'a> {
    pub fn cosine(v: &Vec3) -> Self {
        PDF::Cosine(cosine::Cosine::new(v))
    }

    pub fn sphere() -> Self {
        PDF::Sphere(sphere::Sphere{})
    }

    pub fn hittable(shape: &'a shape::Shape, origin: &Vec3) -> Self {
        PDF::Hittable(hittable::HittablePDF::new(shape, origin))
    }

    pub fn mixture(pdf1: &'a PDF, pdf2: &'a PDF) -> Self {
        PDF::Mixture(mixture::Mixture::new(pdf1, pdf2))
    }
}

impl<'a> ProbabilityDensityFunction for PDF<'a> {
    fn value(&self, direction: &Vec3) -> f32 {
        match self {
            PDF::Cosine(cosine) => cosine.value(direction),
            PDF::Sphere(sphere) => sphere.value(direction),
            PDF::Mixture(mixture) => mixture.value(direction),
            PDF::Hittable(hittable) => hittable.value(direction),
        }
    }

    fn generate(&self) -> Vec3 {
        match self {
            PDF::Cosine(cosine) => cosine.generate(),
            PDF::Sphere(sphere) => sphere.generate(),
            PDF::Mixture(mixture) => mixture.generate(),
            PDF::Hittable(hittable) => hittable.generate(),
        }
    }
}
