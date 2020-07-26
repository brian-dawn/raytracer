use crate::camera::Camera;
use crate::ray::Ray;
use crate::shapes::hittable::{HitRecord, Hittable};
use crate::shapes::hittable_list::HittableList;
use crate::shapes::sphere::Sphere;
use crate::vec3::{Color, Point3, Vec3};

pub fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::new();

    // If we've exceeded the ray bounce limit then we're done gathering light.
    if depth <= 0 {
        return Color::zero();
    }

    if world.hit(r, 0.001, std::f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation = Color::zero();
        if let Some(mat_ptr) = &rec.mat_ptr {
            if mat_ptr.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, &world, depth - 1);
            }
        }
        return Color::zero();
    }

    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
