use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rayon::prelude::*;

use raytracer::camera::Camera;
use raytracer::materials;
use raytracer::render::ray_color;
use raytracer::shapes::hittable::{HitRecord, Hittable};
use raytracer::shapes::hittable_list::HittableList;
use raytracer::shapes::sphere::Sphere;
use raytracer::utils;
use raytracer::vec3::{Color, Point3, Vec3};
use std::sync::Arc;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let material_ground = materials::lambertian::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(material_ground)),
    )));

    for a in -11..11 {
        let af = a as f64;
        let bf = a as f64;

        let choose_mat = 10.0 / (1 + a % 10) as f64;

        let center = Point3::new(af + 0.9, 0.2, bf + 0.9);
        if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
            let sphere_material: Option<Arc<dyn materials::Material + Sync + Send>> =
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::new(0.5, 0.7, 0.0);
                    Some(Arc::new(materials::lambertian::Lambertian { albedo }))
                } else if choose_mat < 0.95 {
                    // Metal

                    let albedo = Color::new(0.5, 0.7, 0.0);
                    let fuzz = 0.3;

                    Some(Arc::new(materials::metal::Metal { albedo, fuzz }))
                } else {
                    // Glass
                    Some(Arc::new(materials::dielectric::Dielectric { ref_idx: 1.5 }))
                };

            world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        }
    }

    world
}

fn render() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    // Image
    let image_width = 50;
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;
    let samples_per_pixel = 15;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        dist_to_focus,
    );

    for j in (0..image_height).rev() {
        (0..image_width)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::zero();
                for s in 0..samples_per_pixel {
                    let u = (i as f64 + utils::random()) / (image_width - 1) as f64;
                    let v = (j as f64 + utils::random()) / (image_height - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, max_depth);
                }
                pixel_color
            })
            .collect::<Vec<Color>>()
            .into_iter()
            .for_each(|pixel_color| {
                // DO NOTHING
            });
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render", |b| b.iter(|| render()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
