use nalgebra::Vector3;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + (self.direction * t)
    }
}
