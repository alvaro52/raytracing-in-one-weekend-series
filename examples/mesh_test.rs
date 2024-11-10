use raytracer::camera::*;
use raytracer::scene::*;
use raytracer::shape::builder::CameraBuilder;
use raytracer::shape::*;
use raytracer::shape::mesh::Mesh;
use raytracer::texture::Texture;
use raytracer::ui;

fn main() -> eframe::Result {
    let mut scene = Scene::new();
    scene.samples = 7000;
    scene.background_color = Vec3::ZERO;
    scene.camera = CameraBuilder::default()
        .with_aspect_ratio(1.0)
        .with_position(Vec3::new(278.0, 278.0, -800.0))
        .looking_at(Vec3::new(278.0, 278.0, 0.0))
        .with_vfov(40.0)
        .build();
    let ground_texture =
        Texture::checkers_from_vec3(Vec3::new(0.2, 0.3, 0.1), Vec3::splat(0.9), 150.0);
    let ground_material = Material::metal(ground_texture, 0.05);
    let light = Material::diffuse_light(Vec3::splat(7.0));
    let red = Material::metal_from_vec3(Vec3::new(0.65, 0.05, 0.05), 0.03);
    let green = Material::metal_from_vec3(Vec3::new(0.12, 0.45, 0.15), 0.03);
    let white = Material::lambertian_from_vec3(Vec3::splat(0.73));
    let gold = Material::lambertian_from_vec3(Vec3::new(0.75164, 0.60648, 0.22648));

    scene.world.push(Shape::quadrilateral(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    scene.world.push(Shape::quadrilateral(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    scene.world.push(Shape::quadrilateral(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    ));
    scene.world.push(Shape::quadrilateral(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        ground_material,
    ));
    scene.world.push(Shape::quadrilateral(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    scene.world.push(Shape::quadrilateral(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white,
    ));

    let mut dragon = Mesh::new("assets/dragon.obj", gold);
    dragon.scale(Vec3::splat(500.0));
    dragon.rotate_y(-60.0);
    dragon.translate(Vec3::new(280.0, 140.0, 190.0));

    scene.world.push(Shape::Mesh(dragon));

    ui::App::run(scene)
}
