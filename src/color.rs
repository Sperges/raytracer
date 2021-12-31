use crate::prelude::*;

pub fn write_color(pixel_color: Color, samples: f64) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples;
    r *= scale;
    g *= scale;
    b *= scale;

    println!(
        "{} {} {}",
        (255.999 * clamp(r, 0.0, 0.999)) as f64,
        (255.999 * clamp(g, 0.0, 0.999)) as f64,
        (255.999 * clamp(b, 0.0, 0.999)) as f64
    );
}