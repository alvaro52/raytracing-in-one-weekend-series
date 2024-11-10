use raytracer::camera::*;
use raytracer::scene::*;
use raytracer::shape::builder::CameraBuilder;
use raytracer::shape::*;
use raytracer::texture::*;
use raytracer::ui;

fn main() -> eframe::Result {
    let mut scene = Scene::new();
    scene.camera = CameraBuilder::default()
        .with_aspect_ratio(1.0)
        .with_position(Vec3::new(0.0, 0.0, 9.0))
        .looking_at(Vec3::ZERO)
        .with_vfov(80.0)
        .build();

    let upper_orange = Material::lambertian_from_vec3(Vec3::new(1.0, 0.5, 0.0));
    let left_red = Material::lambertian_from_vec3(Vec3::new(1.0, 0.2, 0.2));
    let back_green = Material::lambertian_from_vec3(Vec3::new(0.2, 1.0, 0.2));
    let right_blue = Material::lambertian_from_vec3(Vec3::new(0.2, 0.2, 1.0));
    let lower_teal = Material::lambertian_from_vec3(Vec3::new(0.2, 0.8, 0.8));

    scene.world.push(Shape::quadrilateral(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    scene.world.push(Shape::quadrilateral(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    scene.world.push(Shape::quadrilateral(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    scene.world.push(Shape::quadrilateral(
        Vec3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    ));
    scene.world.push(Shape::quadrilateral(
        Vec3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    ));

    ui::App::run(scene)
}
