use raytracer::camera::*;
use raytracer::scene::*;
use raytracer::shape::builder::CameraBuilder;
use raytracer::shape::*;
use raytracer::shape::mesh::Mesh;
use raytracer::texture::Texture;
use raytracer::ui;

fn main() -> eframe::Result {
    let mut scene = Scene::new();
    scene.samples = 500;
    scene.background_color = Vec3::ZERO;
    scene.camera = CameraBuilder::default()
        .with_aspect_ratio(1.0)
        .with_position(Vec3::new(278.0, 278.0, -800.0))
        .looking_at(Vec3::new(278.0, 278.0, 0.0))
        .with_vfov(40.0)
        .build();

    let ground_texture =
        Texture::checkers_from_vec3(Vec3::new(0.2, 0.3, 0.1), Vec3::splat(0.9), 112.0);

    let light = Material::diffuse_light(Vec3::splat(20.0));
    let red = Material::lambertian_from_vec3(Vec3::new(0.65, 0.05, 0.05));
    let green = Material::lambertian_from_vec3(Vec3::new(0.12, 0.45, 0.15));
    let white = Material::lambertian_from_vec3(Vec3::splat(0.73));
    let gold = Material::lambertian_from_vec3(Vec3::new(
        0.8313725490196078,
        0.6862745098039216,
        0.2156862745098039
    ));
    let glass = Material::dielectric(1.5);
    let ground = Material::metal(ground_texture, 0.1);
    let empty = Material::default();

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
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
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

    // let mut cube = Mesh::new("assets/dragon.obj", gold);
    // cube.scale(Vec3::splat(500.0));
    // cube.rotate_y(-70.0);
    // cube.translate(Vec3::new(280.0, 140.0, 300.0));
    //
    // scene.world.push(Shape::Mesh(cube));

    let sphere = Shape::sphere(Vec3::new(190.0, 90.0, 190.0), 90.0, glass);

    scene.world.push(sphere);

    let mut cube = Mesh::new("assets/cube.obj", white.clone());
    cube.scale(Vec3::new(160.0, 320.0, 160.0));
    cube.rotate_y(15.0);
    cube.translate(Vec3::new(340.0, 160.0, 360.0));

    scene.world.push(Shape::Mesh(cube));

    scene.light = Some(
        Shape::list(
            vec![
                Shape::quadrilateral(
                    Vec3::new(343.0, 554.0, 332.0),
                    Vec3::new(-130.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, -105.0),
                    empty.clone()
                ),
                Shape::sphere(
                    Vec3::new(190.0, 90.0, 190.0),
                    90.0,
                    empty
                )
            ]
        )
    );

    ui::App::run(scene)
}