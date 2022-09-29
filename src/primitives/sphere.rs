use crate::*;
use std::borrow::Borrow;

pub struct Sphere {
    centre: Vec3d,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub const fn new(centre: Vec3d, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            centre,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let root = discriminant.sqrt();
            let mut t = (-half_b - root) / a;
            if t < t_min || t > t_max {
                t = (-half_b + root) / a;
                if t < t_min || t > t_max {
                    return None;
                }
            }
            let point = ray.at(t);
            let normal = (point - self.centre) / self.radius;
            let mut hit_record = HitRecord::new(point, normal, t, self.material.borrow());
            hit_record.set_face_normal(ray, normal);
            Some(hit_record)
        }
    }
}
