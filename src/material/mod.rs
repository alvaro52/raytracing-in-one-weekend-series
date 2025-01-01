pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub use crate::camera::*;
use crate::pdf::PDF;
use crate::shape::hittable::HitRecord;
use crate::texture::*;

#[derive(Clone)]
pub enum Material {
    Metal(metal::Metal),
    Isotropic(isotropic::Isotropic),
    Lambertian(lambertian::Lambertian),
    Dielectric(dielectric::Dielectric),
    DiffuseLight(diffuse_light::DiffuseLight),
}

pub struct Scattered<'a> {
    pub attenuation: Vec3,
    pub scattered: Ray,
    pub pdf: Option<PDF<'a>>
}

pub trait Scatters {
    fn scatters(&self, _hit_record: &HitRecord) -> Option<Scattered> {
        None
    }
    fn emitted(&self, _hit_record: &HitRecord) -> Vec3 {
        Vec3::ZERO
    }
    fn scattering_pdf(&self, _ray: &Ray, _hit_record: &HitRecord, _scattered: &Ray) -> f32 {
        0.0
    }
}

impl Scatters for Material {
    fn scatters(&self, hit_record: &HitRecord) -> Option<Scattered> {
        match self {
            Material::Metal(metal) => metal.scatters(hit_record),
            Material::Isotropic(isotropic) => isotropic.scatters(hit_record),
            Material::Dielectric(dielectric) => dielectric.scatters(hit_record),
            Material::Lambertian(lambertian) => lambertian.scatters(hit_record),
            Material::DiffuseLight(diffuse_light) => diffuse_light.scatters(hit_record),
        }
    }

    fn emitted(&self, hit_record: &HitRecord) -> Vec3 {
        match self {
            Material::Metal(metal) => metal.emitted(hit_record),
            Material::Isotropic(isotropic) => isotropic.emitted(hit_record),
            Material::Dielectric(dielectric) => dielectric.emitted(hit_record),
            Material::Lambertian(lambertian) => lambertian.emitted(hit_record),
            Material::DiffuseLight(diffuse_light) => diffuse_light.emitted(hit_record),
        }
    }

    fn scattering_pdf(&self, ray: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32 {
        match self {
            Material::Metal(metal) => metal.scattering_pdf(ray, hit_record, scattered),
            Material::Isotropic(isotropic) => isotropic.scattering_pdf(ray, hit_record, scattered),
            Material::Dielectric(dielectric) => dielectric.scattering_pdf(ray, hit_record, scattered),
            Material::Lambertian(lambertian) => lambertian.scattering_pdf(ray, hit_record, scattered),
            Material::DiffuseLight(diffuse_light) => diffuse_light.scattering_pdf(ray, hit_record, scattered),
        }
    }
}

impl Material {
    pub fn lambertian(albedo: Texture) -> Self {
        Material::Lambertian(lambertian::Lambertian { albedo })
    }

    pub fn lambertian_from_vec3(albedo: Vec3) -> Self {
        Material::Lambertian(lambertian::Lambertian {
            albedo: Texture::solid_color(albedo),
        })
    }

    pub fn metal_from_vec3(albedo: Vec3, fuzz: f32) -> Self {
        Material::Metal(metal::Metal {
            albedo: Texture::solid_color(albedo),
            fuzz: fuzz.min(1.0),
        })
    }

    pub fn metal(albedo: Texture, fuzz: f32) -> Self {
        Material::Metal(metal::Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        })
    }

    pub fn dielectric(refraction_index: f32) -> Self {
        Material::Dielectric(dielectric::Dielectric { refraction_index })
    }

    pub fn diffuse_light(emission: Vec3) -> Self {
        Material::DiffuseLight(diffuse_light::DiffuseLight {
            emission: Texture::solid_color(emission),
        })
    }

    pub fn isotropic_from_vec3(albedo: Vec3) -> Self {
        Material::Isotropic(isotropic::Isotropic {
            albedo: Texture::solid_color(albedo),
        })
    }

    pub fn isotropic(albedo: Texture) -> Self {
        Material::Isotropic(isotropic::Isotropic { albedo })
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::lambertian_from_vec3(Vec3::ONE)
    }
}
