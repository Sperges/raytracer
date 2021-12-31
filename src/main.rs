mod ray;
mod vec3;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;


mod prelude {
    pub use crate::ray::*;
    pub use crate::vec3::*;
    pub use crate::hittable::*;
    pub use crate::sphere::*;
    pub use crate::hittable_list::*;
    pub use crate::rtweekend::*;
}

use prelude::*;

fn write_color(pixel_color: Color) {
    let ir = (255.999 * pixel_color.x) as f64;
    let ig = (255.999 * pixel_color.y) as f64;
    let ib = (255.999 * pixel_color.z) as f64;
    println!("{} {} {}", ir, ig, ib);
}

fn ray_color(ray: &mut Ray, world: &mut HittableList) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(ray, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0))
    }

    let unit_direction: Vec3 = ray.dir.unit_vector();
    let t = 0.5*(unit_direction.y + 1.0);
    return (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

    let width = IMAGE_WIDTH as f64;
    let height = IMAGE_HEIGHT as f64;

    //World
    let mut world = HittableList::new();
    world.add_sphere(Sphere::new(Point3::from(0.0, -100.5, -1.0), 100.0));
    world.add_sphere(Sphere::new(Point3::from(0.0, 0.0, -1.0), 0.5));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new();
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (width - 1.0);
            let v = j as f64 / (height - 1.0);
            let mut ray: Ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&mut ray, &mut world);
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}
