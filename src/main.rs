use std::fs;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    fs::write("output.ppm", "")?;
    Ok(())
}