use raytracer::camera::*;
use raytracer::scene::*;
use raytracer::shape::builder::CameraBuilder;
use raytracer::shape::*;
use raytracer::texture::*;
use raytracer::ui;

fn main() -> eframe::Result {
    let mut scene = Scene::new();
    scene.background_color = Vec3::ZERO;

    scene.camera = CameraBuilder::default()
        .with_position(Vec3::new(13.0, 2.0, 3.0))
        .looking_at(Vec3::ZERO)
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

    ui::App::run(scene)
}
