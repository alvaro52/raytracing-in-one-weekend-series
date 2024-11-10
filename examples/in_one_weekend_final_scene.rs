use rand::Rng;
use raytracer::camera::*;
use raytracer::scene::*;
use raytracer::shape::builder::CameraBuilder;
use raytracer::shape::*;
use raytracer::ui;
use raytracer::util;

fn main() -> eframe::Result {
    let mut scene = Scene::new();
    scene.samples = 5;

    scene.camera = CameraBuilder::default()
        .with_image_height(1000)
        .with_position(Vec3::new(13.0, 2.0, 3.0))
        .looking_at(Vec3::new(0.0, 0.0, 0.0))
        .with_vfov(20.0)
        .with_defocus_angle(0.6)
        .with_focus_dist(10.0)
        .build();

    let ground_material = Material::lambertian_from_vec3(Vec3::splat(0.5));
    scene.world.push(Shape::plane(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground_material,
    ));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.random::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.random::<f32>(),
            );

            match choose_mat {
                0.0..=0.8 => {
                    let albedo = util::random_vec3() * util::random_vec3();
                    let sphere_material = Material::lambertian_from_vec3(albedo);
                    scene
                        .world
                        .push(Shape::sphere(center, 0.2, sphere_material));
                }
                0.8..=0.95 => {
                    let albedo = util::random_vec3_in_range(0.5, 1.0);
                    let fuzz = rng.random::<f32>() * 0.5;
                    let sphere_material = Material::metal_from_vec3(albedo, fuzz);
                    scene
                        .world
                        .push(Shape::sphere(center, 0.2, sphere_material));
                }
                _ => {
                    let sphere_material = Material::dielectric(1.5);
                    scene
                        .world
                        .push(Shape::sphere(center, 0.2, sphere_material));
                }
            }
        }

        let dielectric = Material::dielectric(1.5);
        scene
            .world
            .push(Shape::sphere(Vec3::new(0.0, 1.0, 0.0), 1.0, dielectric));

        let lambertian = Material::lambertian_from_vec3(Vec3::new(0.4, 0.2, 0.1));
        scene
            .world
            .push(Shape::sphere(Vec3::new(-4.0, 1.0, 0.0), 1.0, lambertian));

        let metal = Material::metal_from_vec3(Vec3::new(0.7, 0.6, 0.5), 0.0);
        scene
            .world
            .push(Shape::sphere(Vec3::new(4.0, 1.0, 0.0), 1.0, metal));
    }

    ui::App::run(scene)
}
