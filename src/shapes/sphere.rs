use std::sync::Arc;

use super::hittable::Hittable;
use crate::materials::Material;
use crate::vec3::Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn Material + Sync + Send>>,
}

impl Sphere {
    pub fn new(
        center: Point3,
        radius: f64,
        mat_ptr: Option<Arc<dyn Material + Sync + Send>>,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        let oc = r.origin - self.center;

        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(&r, &outward_normal);

                // TODO: I don't like this clone here but maybe it's OK
                rec.mat_ptr = self.mat_ptr.clone();

                return true;
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(&r, &outward_normal);

                // TODO: I don't like this clone here but maybe it's OK
                rec.mat_ptr = self.mat_ptr.clone();

                return true;
            }
        }

        false
    }
}
