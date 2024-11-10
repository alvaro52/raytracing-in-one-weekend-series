use raytracer::camera::*;
use raytracer::scene::*;
use raytracer::shape::builder::CameraBuilder;
use raytracer::shape::*;
use raytracer::texture::*;
use raytracer::ui;

fn main() -> eframe::Result {
    let mut scene = Scene::new();
    scene.samples = 100;
    scene.background_color = Vec3::ZERO;
    scene.camera = CameraBuilder::default()
        .with_position(Vec3::new(26.0, 3.0, 6.0))
        .looking_at(Vec3::new(0.0, 2.0, 0.0))
        .with_vfov(20.0)
        .build();

    let noise_texture = Texture::perlin(4.0);
    let sphere_material = Material::lambertian(noise_texture.clone());
    scene.world.push(Shape::sphere(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        sphere_material,
    ));

    let ground_material = Material::lambertian(noise_texture);
    scene.world.push(Shape::plane(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    ));

    let diffuse_light = Material::diffuse_light(Vec3::splat(4.0));
    scene.world.push(Shape::quadrilateral(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        diffuse_light.clone(),
    ));
    scene
        .world
        .push(Shape::sphere(Vec3::new(0.0, 7.0, 0.0), 2.0, diffuse_light));

    ui::App::run(scene)
}
