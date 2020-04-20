use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::io::prelude::*;

mod pixel;
mod ray;

fn main() -> Result<(), anyhow::Error> {
    const WIDTH: u64 = 200;
    const HEIGHT: u64 = 100;

    let mut file = std::fs::File::create("image.ppm")?;
    write!(file, "P3\n{} {}\n255\n", WIDTH, HEIGHT)?;
    (0..HEIGHT)
        .rev()
        .flat_map(|y| (0..WIDTH).map(move |x| (x, y)))
        .progress_with({
            let t = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})";
            let s = ProgressStyle::default_bar().template(t);
            ProgressBar::new(HEIGHT * WIDTH).with_style(s)
        })
        .map(|(x, y)| {
            pixel::Pixel::new(x as f64 / WIDTH as f64, y as f64 / HEIGHT as f64,0.2)
        })
        .map(|px| file.write(px.to_ppm_color().as_bytes()).map(drop))
        .collect::<Result<_, _>>()?;

    Ok(())
}
