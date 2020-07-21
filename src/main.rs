use std::io::Write;

pub mod ray;
pub mod vec3;

use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn write_color(pixel: &Color) {
    let ir = (255.999 * pixel.x) as i32;
    let ig = (255.999 * pixel.y) as i32;
    let ib = (255.999 * pixel.z) as i32;

    println!("{} {} {}", ir, ig, ib);
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;

    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let mut t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        // normal
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = r.direction.unit();
    t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::ones() + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    // Image

    let image_width = 256;
    let image_height = (image_width as f64 / ASPECT_RATIO) as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&r);
            write_color(&pixel_color);
        }
    }

    eprint!("\nDone.\n");
}
