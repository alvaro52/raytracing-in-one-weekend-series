use glam::f32::Vec3;
use rand::Rng;

pub fn random_vec3() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.random::<f32>(),
        rng.random::<f32>(),
        rng.random::<f32>(),
    )
}

pub fn random_vec3_in_range(min: f32, max: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let vec = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );

        if vec.length_squared() <= 1.0 {
            return vec;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let random = random_unit_vector();
    if normal.dot(random) < 0.0 {
        -random
    } else {
        random
    }
}

pub fn random_cosine_direction() -> Vec3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.random::<f32>();
    let r2 = rng.random::<f32>();

    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let z = (1.0 - r2).sqrt();

    Vec3::new(x, y, z)
}

pub fn near_zero(vec: Vec3) -> bool {
    const EPSILON: f32 = 1e-8;
    vec.x.abs() < EPSILON && vec.y.abs() < EPSILON && vec.z.abs() < EPSILON
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let vec = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);

        if vec.length_squared() < 1.0 {
            return vec;
        }
    }
}
