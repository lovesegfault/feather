mod pixel;
mod ray;
use pixel::Pixel;
use ray::Ray;

use nalgebra::Point3;
use nalgebra::Vector3;
use rayon::prelude::*;
use std::io::prelude::*;

fn hit_sphere(center: Point3<f64>, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.norm_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.norm_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - (a * c);
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
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
    const WIDTH: usize = 2000;
    const HEIGHT: usize = 1000;

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);

    let mut img: Vec<Pixel> = vec![Pixel::default(); WIDTH * HEIGHT];
    img.par_iter_mut().enumerate().for_each(|(i, px)| {
        let (x, y) = (i % WIDTH, HEIGHT - i / WIDTH + 1);
        let u = x as f64 / WIDTH as f64;
        let v = y as f64 / HEIGHT as f64;
        let r = Ray::new(
            Point3::origin(),
            lower_left_corner + u * horizontal + v * vertical,
        );
        *px = ray_color(r);
    });

    let mut file = std::fs::File::create("image.ppm")?;
    write!(file, "P3\n{} {}\n255\n", WIDTH, HEIGHT)?;
    img.into_iter()
        .map(|px| file.write(&px.to_ppm_color().into_bytes()).map(drop))
        .collect::<Result<_, _>>()?;

    Ok(())
}
