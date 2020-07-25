use super::Material;
use crate::{ray::Ray, shapes::hittable::HitRecord, vec3::Color, vec3::Vec3};

pub struct Lambertian {
    pub albedo: Color,
}


impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
