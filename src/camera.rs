use crate::*;

pub struct Camera {
    origin: Vec3d,
    lower_left_corner: Vec3d,
    horizontal: Vec3d,
    vertical: Vec3d,
}

impl Camera {
    pub fn new() -> Self {
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * ASPECT_RATIO;
        let focal_length: f64 = 1.0;

        let origin = Vec3d::new(0.0, 0.0, 0.0);
        let horizontal = Vec3d::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3d::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3d::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
