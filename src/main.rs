mod vec3;

fn main() {
    const IMAGE_HEIGHT: usize = 256;
    const IMAGE_WIDTH: usize = 256;

    let width = IMAGE_WIDTH as f64;
    let height = IMAGE_HEIGHT as f64;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (width - 1.0);
            let g = j as f64 / (height - 1.0);
            let b = 0.25;

            let ir = (255.999 * r) as f64;
            let ig = (255.999 * g) as f64;
            let ib = (255.999 * b) as f64;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }

    eprintln!("\nDone.");
}