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
