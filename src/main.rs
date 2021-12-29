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

// fn ray_color(ray: Ray) -> Color {
//     let unit_direction: Vec3 = ray.dir;
// }

fn main() {
    // Image
    const IMAGE_HEIGHT: usize = 256;
    const IMAGE_WIDTH: usize = 256;

    let width = IMAGE_WIDTH as f64;
    let height = IMAGE_HEIGHT as f64;

    // Render

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color: Color =
                Color::new(i as f64 / (width - 1.0), j as f64 / (height - 1.0), 0.25);
            write_color(pixel_color);
        }
    }

    eprintln!("\nDone.");
}
