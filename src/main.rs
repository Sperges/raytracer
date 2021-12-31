mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

mod prelude {
    pub use crate::camera::*;
    pub use crate::color::*;
    pub use crate::hittable::*;
    pub use crate::hittable_list::*;
    pub use crate::ray::*;
    pub use crate::rtweekend::*;
    pub use crate::sphere::*;
    pub use crate::vec3::*;
}

use prelude::*;

fn ray_color(ray: &mut Ray, world: &mut HittableList) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(ray, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
    }

    let unit_direction: Vec3 = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
    const SAMPLES_PER_PIXEL: i64 = 100;

    let width = IMAGE_WIDTH as f64;
    let height = IMAGE_HEIGHT as f64;
    let samples = SAMPLES_PER_PIXEL as f64;

    //World
    let mut world = HittableList::new();
    world.add_sphere(Sphere::new(Point3::from(0.0, -100.5, -1.0), 100.0));
    world.add_sphere(Sphere::new(Point3::from(0.0, 0.0, -1.0), 0.5));

    // Camera
    let camera = Camera::new();

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (width - 1.0);
                let v = (j as f64 + random_f64()) / (height - 1.0);
                let mut ray = camera.get_ray(u, v);
                pixel_color += ray_color(&mut ray, &mut world);
            }
            write_color(pixel_color, samples);
        }
    }

    eprintln!("\nDone.");
}
