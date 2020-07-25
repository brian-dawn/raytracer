use std::io::Write;
use std::rc::Rc;

pub mod camera;
pub mod materials;
pub mod ray;
pub mod shapes;
pub mod utils;
pub mod vec3;

use camera::Camera;
use ray::Ray;
use shapes::hittable::{HitRecord, Hittable};
use shapes::hittable_list::HittableList;
use shapes::sphere::Sphere;
use vec3::{Color, Point3, Vec3};

fn write_color(pixel: &Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;

    // Divide color by the number of samples and gamma correct for gamma=2.0.
    let r = (pixel.x * scale).sqrt();
    let g = (pixel.y * scale).sqrt();
    let b = (pixel.z * scale).sqrt();

    let ir = (256.0 * utils::clamp(r, 0.0, 0.999)) as i32;
    let ig = (256.0 * utils::clamp(g, 0.0, 0.999)) as i32;
    let ib = (256.0 * utils::clamp(b, 0.0, 0.999)) as i32;

    println!("{} {} {}", ir, ig, ib);
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
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

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    // Image
    let image_width = 256;
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let r = (std::f64::consts::PI / 4.0).cos();

    let mut world = HittableList::new();

    let material_ground = materials::lambertian::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };

    let material_center = materials::lambertian::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };

    let material_left = materials::dielectric::Dielectric { ref_idx: 1.5 };

    let material_right = materials::metal::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Some(Rc::new(material_ground)),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Some(Rc::new(material_center)),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(Rc::new(material_left)),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        Some(Rc::new(material_left)),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Some(Rc::new(material_right)),
    )));

    // Camera
    let cam = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
    );

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color = Color::zero();
            for s in 0..samples_per_pixel {
                let u = (i as f64 + utils::random()) / (image_width - 1) as f64;
                let v = (j as f64 + utils::random()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            write_color(&pixel_color, samples_per_pixel);
        }
    }

    eprint!("\nDone.\n");
}
