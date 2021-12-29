mod ray;
mod vec3;

mod prelude {
    pub use crate::ray::*;
    pub use crate::vec3::*;
}

use prelude::*;

fn write_color(pixel_color: Color) {
    let ir = (255.999 * pixel_color.x) as f64;
    let ig = (255.999 * pixel_color.y) as f64;
    let ib = (255.999 * pixel_color.z) as f64;
    println!("{} {} {}", ir, ig, ib);
}

fn ray_color(ray: Ray) -> Color {
    let unit_direction: Vec3 = ray.dir.unit_vector();
    let t: f64 = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

    let width = IMAGE_WIDTH as f64;
    let height = IMAGE_HEIGHT as f64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (width - 1.0);
            let v = j as f64 / (height - 1.0);
            let ray: Ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(ray);
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}
