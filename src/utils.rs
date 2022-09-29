use rand::Rng;
use spyder_math::Vec3d;

pub fn random_new_vec3d() -> Vec3d {
    let lower: f64 = -1.0;
    let upper: f64 = 1.0;
    Vec3d::new(
        rand::thread_rng().gen_range(lower..upper),
        rand::thread_rng().gen_range(lower..upper),
        rand::thread_rng().gen_range(lower..upper),
    )
    .normalise()
}

pub fn reflect_on_normal(incident: Vec3d, normal: Vec3d) -> Vec3d {
    incident - normal * 2.0 * incident.dot(normal)
}

pub fn refract(incident: Vec3d, normal: Vec3d, etai_over_etat: f64) -> Vec3d {
    let cos_theta = (-incident).dot(normal).min(1.0);
    let r_out_perp = etai_over_etat * (incident + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
