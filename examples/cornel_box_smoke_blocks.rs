use raytracer::camera::*;
use raytracer::scene::*;
use raytracer::shape::builder::CameraBuilder;
use raytracer::shape::*;
use raytracer::shape::mesh::Mesh;
use raytracer::ui;

fn main() -> eframe::Result {
    let mut scene = Scene::new();
    scene.samples = 100;
    scene.background_color = Vec3::ZERO;
    scene.camera = CameraBuilder::default()
        .with_aspect_ratio(1.0)
        .with_position(Vec3::new(278.0, 278.0, -800.0))
        .looking_at(Vec3::new(278.0, 278.0, 0.0))
        .with_vfov(40.0)
        .build();

    let light = Material::diffuse_light(Vec3::splat(7.0));
    let red = Material::lambertian_from_vec3(Vec3::new(0.65, 0.05, 0.05));
    let green = Material::lambertian_from_vec3(Vec3::new(0.12, 0.45, 0.15));
    let white = Material::lambertian_from_vec3(Vec3::splat(0.73));

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
        white.clone(),
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
        white.clone(),
    ));

    let mut cube = Mesh::new("assets/cube.obj", white.clone());
    cube.scale(Vec3::new(160.0, 160.0, 160.0));
    cube.rotate_y(-15.0);
    cube.translate(Vec3::new(210.0, 80.0, 180.0));

    scene.world.push(Shape::smoke_box(cube, 0.01, Vec3::splat(0.0)));

    let mut cube = Mesh::new("assets/cube.obj", white.clone());
    cube.scale(Vec3::new(160.0, 320.0, 160.0));
    cube.rotate_y(15.0);
    cube.translate(Vec3::new(340.0, 160.0, 360.0));

    scene.world.push(Shape::smoke_box(cube, 0.01, Vec3::splat(1.0)));

    ui::App::run(scene)
}
