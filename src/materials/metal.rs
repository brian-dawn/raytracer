use super::Material;
use crate::{ray::Ray, shapes::hittable::HitRecord, vec3::Color, vec3::Vec3};

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction.unit().reflect(&rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        scattered.direction.dot(&rec.normal) > 0.0
    }
}
