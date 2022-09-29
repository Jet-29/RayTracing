use crate::*;

pub struct HitRecord<'a> {
    pub point: Vec3d,
    pub normal: Vec3d,
    pub t: f64,
    pub material: &'a dyn Material,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub const fn new(point: Vec3d, normal: Vec3d, t: f64, material: &'a dyn Material) -> Self {
        Self {
            point,
            normal,
            t,
            material,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3d) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_result: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_result = Some(hit_record);
            }
        }

        hit_result
    }
}

pub struct Ray {
    pub origin: Vec3d,
    pub direction: Vec3d,
}

impl Ray {
    pub fn new(origin: Vector<f64, 3>, direction: Vector<f64, 3>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector<f64, 3> {
        self.origin + self.direction * t
    }

    pub fn ray_colour<T: Hittable>(&self, hittable: &T, depth: u32) -> Colour {
        if let Some(hit_result) = hittable.hit(self, 1.0e-6, f64::INFINITY) {
            return if depth > 0 {
                if let Some(scatter_record) = hit_result.material.scatter(self, &hit_result) {
                    Colour::multiply_colour(
                        scatter_record.attenuation,
                        scatter_record.scattered.ray_colour(hittable, depth - 1),
                    )
                } else {
                    Colour::new(0.0, 0.0, 0.0)
                }
            } else {
                Colour::new(0.0, 0.0, 0.0)
            };
        }

        let unit_direction = self.direction.normalise();
        let t = 0.5 * (unit_direction.y + 1.0);
        Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
    }
}
