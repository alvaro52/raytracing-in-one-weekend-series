use raytracer::camera::*;
use raytracer::scene::*;
use raytracer::shape::*;
use raytracer::ui;

fn main() -> eframe::Result {
    let mut scene = Scene::new();

    let material_ground = Material::lambertian_from_vec3(Vec3::new(0.8, 0.8, 0.0));
    let material_middle = Material::lambertian_from_vec3(Vec3::new(0.1, 0.2, 0.5));
    let material_right = Material::metal_from_vec3(Vec3::new(0.8, 0.6, 0.2), 1.0);
    let material_left = Material::dielectric(1.5);
    let material_bubble = Material::dielectric(1.00 / 1.50);

    scene.world.push(Shape::sphere(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    scene.world.push(Shape::sphere(
        Vec3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    scene.world.push(Shape::sphere(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));
    scene.world.push(Shape::sphere(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_middle,
    ));
    scene.world.push(Shape::plane(
        Vec3::new(0.0, -0.5, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        material_ground,
    ));

    ui::App::run(scene)
}
