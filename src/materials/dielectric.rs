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

        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            // Must reflect.
            let reflected = unit_direction.reflect(&rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }
        // Can refract.

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if crate::utils::random() < reflect_prob {
            let reflected = unit_direction.reflect(&rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }

        let refracted = unit_direction.refract(&rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted);

        true
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
