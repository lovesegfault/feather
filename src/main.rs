mod pixel;
mod ray;
use pixel::Pixel;
use ray::Ray;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use nalgebra::Point3;
use nalgebra::Vector3;
use std::io::prelude::*;

fn ray_color(ray: Ray) -> Pixel {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction[1] + 1.0);
    let color = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    color.into()
}

fn main() -> Result<(), anyhow::Error> {
    const WIDTH: u64 = 200;
    const HEIGHT: u64 = 100;

    let mut file = std::fs::File::create("image.ppm")?;
    write!(file, "P3\n{} {}\n255\n", WIDTH, HEIGHT)?;

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);

    (0..HEIGHT)
        .rev()
        .flat_map(|y| (0..WIDTH).map(move |x| (x, y)))
        .progress_with({
            let t = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})";
            let s = ProgressStyle::default_bar().template(t);
            ProgressBar::new(HEIGHT * WIDTH).with_style(s)
        })
        .map(|(x, y)| {
            let u = x as f64 / WIDTH as f64;
            let v = y as f64 / HEIGHT as f64;
            let r = Ray::new(Point3::origin(), lower_left_corner + u * horizontal + v * vertical);
            ray_color(r)
        })
        .map(|px| file.write(px.to_ppm_color().as_bytes()).map(drop))
        .collect::<Result<_, _>>()?;

    Ok(())
}
