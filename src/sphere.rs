use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};
use std::ops::RangeInclusive;

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if t < t_max && t > t_min {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord { point, normal, t });
            }
            let t = (-half_b + root) / a;
            if t < t_max && t > t_min {
                let point = ray.at(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord { point, normal, t });
            }
        }
        None
    }
}
