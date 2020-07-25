use crate::ray::Ray;
use crate::shapes::hittable::HitRecord;
use crate::vec3::Color;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
