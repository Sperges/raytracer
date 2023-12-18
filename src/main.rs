mod camera;
mod color;
mod hit_record;
mod hittable;
mod ray;
mod util;
mod vec3;
mod world;
mod sphere;
mod interval;

mod prelude {
    pub use crate::camera::*;
    pub use crate::color::*;
    pub use crate::hit_record::*;
    pub use crate::hittable::*;
    pub use crate::ray::*;
    pub use crate::util::*;
    pub use crate::vec3::*;
    pub use crate::world::*;
    pub use crate::sphere::*;
	pub use crate::interval::*;
}

use prelude::*;

enum Renderer {
    Lambertian,
    Hemisphere,
}

fn ray_color_lambertian(ray: &mut Ray, world: &mut World, depth: i64) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new();
    }
    
    if world.hit(ray, &Interval::from_f64(0.001, INFINITY), &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        return 0.5 * ray_color_lambertian(&mut Ray::new(rec.p, target - rec.p), world, depth - 1)
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0);
}

fn ray_color_hemisphere(ray: &mut Ray, world: &mut World, depth: i64) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new();
    }
    
    if world.hit(ray, &Interval::from_f64(0.001, INFINITY), &mut rec) {
        let target = rec.p + Vec3::random_in_hemisphere(&rec.normal);
        return 0.5 * ray_color_hemisphere(&mut Ray::new(rec.p, target - rec.p), world, depth - 1)
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;
    const SAMPLES_PER_PIXEL: i64 = 16;
    const MAX_DEPTH: i64 = 6;
    const RENDERER: Renderer = Renderer::Lambertian;

    let width = IMAGE_WIDTH as f64;
    let height = IMAGE_HEIGHT as f64;
    let samples = SAMPLES_PER_PIXEL as f64;

    // World
    let mut world = World::new();
    world.add(Sphere::new(Point3::from(0.0, -100.5, -1.0), 100.0));
    world.add(Sphere::new(Point3::from(0.0, 0.0, -1.0), 0.5));

    // Camera
    let camera = Camera::new();

    // Render
	let mut result = String::new();
    result.push_str(&format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (width - 1.0);
                let v = (j as f64 + random_f64()) / (height - 1.0);
                let mut ray = camera.get_ray(u, v);
                pixel_color += match RENDERER {
                    Renderer::Lambertian => ray_color_lambertian(&mut ray, &mut world, MAX_DEPTH),
                    Renderer::Hemisphere => ray_color_hemisphere(&mut ray, &mut world, MAX_DEPTH),
                }
                
            }
            result.push_str(&write_color(pixel_color, samples));
        }
    }

    eprintln!("\nDone.");
	println!("{}", result);
}
