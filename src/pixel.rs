use nalgebra::Vector3;

#[derive(Clone)]
pub struct Pixel(Vector3<f64>);

impl std::ops::Deref for Pixel {
    type Target = Vector3<f64>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel(Vector3::new(0.0, 0.0, 0.0))
    }
}

impl Pixel {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Pixel(Vector3::new(r, g, b))
    }

    pub fn r(&self) -> f64 {
        self[0]
    }

    pub fn g(&self) -> f64 {
        self[1]
    }

    pub fn b(&self) -> f64 {
        self[2]
    }

    pub fn to_ppm_color(&self) -> String {
        let r = (255.999 * self.r()) as u64;
        let g = (255.999 * self.g()) as u64;
        let b = (255.999 * self.b()) as u64;
        format!("{} {} {}\n", r, g, b)
    }
}

impl From<Vector3<f64>> for Pixel {
    fn from(v: Vector3<f64>) -> Self {
        Pixel(v)
    }
}
