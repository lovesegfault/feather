use std::io::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    const WIDTH: u64 = 200;
    const HEIGHT: u64 = 100;

    let mut file = std::fs::File::create("image.ppm")?;
    write!(file, "P3\n{} {}\n255\n", WIDTH, HEIGHT)?;
    (0..HEIGHT)
        .rev()
        .flat_map(|y| (0..WIDTH).map(move |x| (x, y)))
        .map(|(x, y)| {
            let r = x as f64 / WIDTH as f64;
            let g = y as f64 / HEIGHT as f64;
            let b = 0.2;
            let ir = (255.999 * r) as u64;
            let ig = (255.999 * g) as u64;
            let ib = (255.999 * b) as u64;
            (ir, ig, ib)
        })
        .map(|(r, g, b)| write!(file, "{} {} {}\n", r, g, b))
        .collect::<Result<_, _>>()?;

    Ok(())
}
