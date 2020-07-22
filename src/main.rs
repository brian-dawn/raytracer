use std::io::Write;
use std::rc::Rc;

pub mod camera;
pub mod ray;
pub mod shapes;
pub mod utils;
pub mod vec3;

use camera::Camera;
use ray::Ray;
use shapes::hittable::{HitRecord, Hittable};
use shapes::hittable_list::HittableList;
use shapes::sphere::Sphere;
use vec3::{Color, Point3};

fn write_color(pixel: &Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = pixel.x * scale;
    let g = pixel.y * scale;
    let b = pixel.z * scale;

    let ir = (256.0 * utils::clamp(r, 0.0, 0.999)) as i32;
    let ig = (256.0 * utils::clamp(g, 0.0, 0.999)) as i32;
    let ib = (256.0 * utils::clamp(b, 0.0, 0.999)) as i32;

    println!("{} {} {}", ir, ig, ib);
}

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(r, 0.0, std::f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
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

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

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
                pixel_color += ray_color(&r, &world);
            }

            write_color(&pixel_color, samples_per_pixel);
        }
    }

    eprint!("\nDone.\n");
}
