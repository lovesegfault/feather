mod pixel;
mod ray;
use pixel::Pixel;
use ray::Ray;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use nalgebra::Point3;
use nalgebra::Vector3;
use rayon::prelude::*;
use std::io::prelude::*;

fn hit_sphere(center: Point3<f64>, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.norm_squared();
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.norm_squared() - radius.powi(2);
    let discriminant = b.powi(2) - (4.0 * a * c);
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(ray: Ray) -> Pixel {
    let t: f64 = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let normal = (ray.at(t) - Point3::new(0.0, 0.0, -1.0)).normalize();
        let color = 0.5 * (normal + Vector3::new(1.0, 1.0, 1.0));
        return color.into();
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction[1] + 1.0);
    let color = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    color.into()
}

fn main() -> Result<(), anyhow::Error> {
    const WIDTH: u32 = 3840;
    const HEIGHT: u32 = 2420;

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);

    let img: Vec<u8> = (0..HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|y| (0..WIDTH).into_par_iter().map(move |x| (x, y)))
        .progress_with({
            let t = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})";
            let s = ProgressStyle::default_bar().template(t);
            let len = (HEIGHT * WIDTH) as u64;
            ProgressBar::new(len).with_style(s)
        })
        .map(|(x, y)| {
            let u = x as f64 / WIDTH as f64;
            let v = y as f64 / HEIGHT as f64;
            let r = Ray::new(Point3::origin(), lower_left_corner + u * horizontal + v * vertical);
            ray_color(r)
        })
        .flat_map(|px| px.to_ppm_color().into_bytes())
        .collect();

    let mut file = std::fs::File::create("image.ppm")?;
    write!(file, "P3\n{} {}\n255\n", WIDTH, HEIGHT)?;
    file.write(&img)?;

    Ok(())
}
