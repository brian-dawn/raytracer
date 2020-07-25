use super::Material;
use crate::{ray::Ray, shapes::hittable::HitRecord, vec3::Color, vec3::Vec3};

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::ones();
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.direction.unit();
        let refracted = unit_direction.refract(&rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted);

        true
    }
}
