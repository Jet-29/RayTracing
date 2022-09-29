use crate::*;

pub struct Camera {
    origin: Vec3d,
    lower_left_corner: Vec3d,
    horizontal: Vec3d,
    vertical: Vec3d,
    u: Vec3d,
    v: Vec3d,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Vec3d,
        look_at: Vec3d,
        up_vector: Vec3d,
        vfov: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = viewport_height * ASPECT_RATIO;

        let w = (look_from - look_at).normalise();
        let u = up_vector.cross(w).normalise();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius
            * Vec3d::new(
                rand::thread_rng().gen_range(-1.0..1.0),
                rand::thread_rng().gen_range(-1.0..1.0),
                0.0,
            );
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
