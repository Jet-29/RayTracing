use std::io::Write;

use rand::Rng;
use spyder_math::*;

use camera::*;
use colour::*;
use ray::*;

use crate::primitives::*;

mod camera;
mod colour;
mod primitives;
mod ray;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (400.0 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 1000;
const SAMPLE_RANGE: f64 = 0.5;

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3d::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3d::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    let mut file = std::fs::File::create("image.ppm")?;
    file.write_all(format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").as_ref())?;

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            // flip image
            let x = (IMAGE_WIDTH - x) as f64;
            let y = (IMAGE_HEIGHT - y) as f64;

            let mut pixel_colour: Colour = Vec3d::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + rng.gen_range(-SAMPLE_RANGE..=SAMPLE_RANGE))
                    / (IMAGE_WIDTH - 1) as f64;
                let v = (y as f64 + rng.gen_range(-SAMPLE_RANGE..=SAMPLE_RANGE))
                    / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_colour += ray.ray_colour(&world);
            }
            pixel_colour.write_ppm(&mut file, SAMPLES_PER_PIXEL);
        }
    }

    Ok(())
}
