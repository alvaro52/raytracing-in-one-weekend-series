pub mod constant_medium;
pub mod triangle;
pub mod hittable;
pub mod mesh;
pub mod plane;
pub mod quadrilateral;
pub mod sphere;

pub use crate::shape::hittable::*;

#[derive(Clone)]
pub enum Shape {
    Quadrilateral(quadrilateral::Quadrilateral),
    SmokeCube(constant_medium::SmokeCube),
    Sphere(sphere::Sphere),
    Plane(plane::Plane),
    Mesh(mesh::Mesh),
    List(Vec<Shape>),
}

impl Hittable for Shape {
    fn hits(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        match self {
            Shape::Quadrilateral(quadrilateral) => quadrilateral.hits(ray, interval),
            Shape::SmokeCube(smoke_cube) => smoke_cube.hits(ray, interval),
            Shape::Sphere(sphere) => sphere.hits(ray, interval),
            Shape::List(shapes) => shapes.hits(ray, interval),
            Shape::Plane(plane) => plane.hits(ray, interval),
            Shape::Mesh(mesh) => mesh.hits(ray, interval),
        }
    }

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f32 {
        match self {
            Shape::Quadrilateral(quadrilateral) => quadrilateral.pdf_value(origin, direction),
            Shape::SmokeCube(smoke_cube) => smoke_cube.pdf_value(origin, direction),
            Shape::Sphere(sphere) => sphere.pdf_value(origin, direction),
            Shape::List(shapes) => shapes.pdf_value(origin, direction),
            Shape::Plane(plane) => plane.pdf_value(origin, direction),
            Shape::Mesh(mesh) => mesh.pdf_value(origin, direction),
        }
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        match self {
            Shape::Quadrilateral(quadrilateral) => quadrilateral.random(origin),
            Shape::SmokeCube(smoke_cube) => smoke_cube.random(origin),
            Shape::Sphere(sphere) => sphere.random(origin),
            Shape::List(shapes) => shapes.random(origin),
            Shape::Plane(plane) => plane.random(origin),
            Shape::Mesh(mesh) => mesh.random(origin),
        }
    }
}

impl Shape {
    pub fn sphere(center: Vec3, radius: f32, material: Material) -> Self {
        Shape::Sphere(sphere::Sphere::new(center, radius, material))
    }

    pub fn moving_sphere(center1: Vec3, center2: Vec3, radius: f32, material: Material) -> Self {
        Shape::Sphere(sphere::Sphere::moving(center1, center2, radius, material))
    }

    pub fn plane(center: Vec3, normal: Vec3, material: Material) -> Self {
        Shape::Plane(plane::Plane {
            center,
            normal,
            material,
            radius: None,
        })
    }

    pub fn quadrilateral(starting_corner: Vec3, u: Vec3, v: Vec3, material: Material) -> Self {
        Shape::Quadrilateral(quadrilateral::Quadrilateral::new(
            starting_corner,
            u,
            v,
            material,
        ))
    }

    pub fn smoke_box(boundary: mesh::Mesh, density: f32, albedo: Vec3) -> Self {
        Shape::SmokeCube(constant_medium::SmokeCube::new(boundary, density, albedo))
    }

    pub fn mesh(path_to_model: &str, material: Material) -> Self {
        Shape::Mesh(mesh::Mesh::new(path_to_model, material))
    }

    pub fn list(shapes: Vec<Shape>) -> Self {
        Shape::List(shapes)
    }
}
