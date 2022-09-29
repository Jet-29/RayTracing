use std::io::Write;

use rand::Rng;
use spyder_math::*;

use camera::*;
use colour::*;
use materials::*;
use ray::*;
use utils::*;

use crate::primitives::*;

mod camera;
mod colour;
mod materials;
mod primitives;
mod ray;
mod utils;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = (400.0 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const SAMPLE_RANGE: f64 = 0.5;
const MAX_DEPTH: u32 = 25;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    let world = random_world();

    let look_from = Vec3d::new(3.10, 3.0, 2.0);
    let look_at = Vec3d::new(0.0, 0.0, 0.0);
    let up_vector = Vec3d::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(look_from, look_at, up_vector, 20.0, aperture, dist_to_focus);

    let mut file = std::fs::File::create("image.ppm")?;
    file.write_all(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_ref())?;

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            if ((y * IMAGE_WIDTH + x) % 1000) == 0 {
                println!(
                    "{}%",
                    ((y * IMAGE_WIDTH + x) * 100) / (IMAGE_HEIGHT * IMAGE_WIDTH)
                );
            }

            // flip image
            let x = x as f64;
            let y = (IMAGE_HEIGHT - y) as f64;

            let mut pixel_colour: Colour = Vec3d::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u =
                    (x + rng.gen_range(-SAMPLE_RANGE..=SAMPLE_RANGE)) / (IMAGE_WIDTH - 1) as f64;
                let v =
                    (y + rng.gen_range(-SAMPLE_RANGE..=SAMPLE_RANGE)) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_colour += ray.ray_colour(&world, MAX_DEPTH);
            }
            pixel_colour.write_ppm(&mut file, SAMPLES_PER_PIXEL);
        }
    }

    Ok(())
}

fn random_world() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(Vec3d::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3d::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for i in -10..=10 {
        for j in -10..=10 {
            let center = Vec3d::new(
                i as f64 + 0.9 * gen_random(),
                0.2,
                j as f64 + 0.9 * gen_random(),
            );

            if (center - Vec3d::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose_mat = gen_random();
                let sphere_material: Box<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Vec3d::new(
                        gen_random() * gen_random(),
                        gen_random() * gen_random(),
                        gen_random() * gen_random(),
                    );
                    sphere_material = Box::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3d::new(gen_random(), gen_random(), gen_random());
                    let fuzz = gen_random_range(0.0, 0.5);
                    sphere_material = Box::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Box::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    world
}

fn gen_random() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

fn gen_random_range(low: f64, high: f64) -> f64 {
    rand::thread_rng().gen_range(low..high)
}
