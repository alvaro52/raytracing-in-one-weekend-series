use raytracer::camera::*;
use raytracer::scene::*;
use raytracer::shape::*;
use raytracer::texture::*;
use raytracer::ui;

fn main() -> eframe::Result {
    let mut scene = Scene::new();

    let sphere_texture = Texture::image_tex("assets/earthmap.jpg");
    let sphere_material = Material::lambertian(sphere_texture);
    scene.world.push(Shape::sphere(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        sphere_material,
    ));

    ui::App::run(scene)
}
