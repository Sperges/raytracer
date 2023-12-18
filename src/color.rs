use crate::prelude::*;

pub fn write_color(pixel_color: Color, samples: f64) -> String {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    format!(
        "{} {} {}\n",
        (255.999 * clamp(r, 0.0, 0.999)) as f64,
        (255.999 * clamp(g, 0.0, 0.999)) as f64,
        (255.999 * clamp(b, 0.0, 0.999)) as f64
    )
}