use std::io::Write;

use spyder_math::*;

pub type Colour = Vec3d;

pub trait WritePpm {
    fn write_ppm(&self, file: &mut impl Write, samples: u32);
}

impl WritePpm for Colour {
    fn write_ppm(&self, file: &mut impl Write, samples: u32) {
        let scale = 1.0 / samples as f64;

        let r = (self.data[0][0] * 255.0 * scale) as u8;
        let g = (self.data[0][1] * 255.0 * scale) as u8;
        let b = (self.data[0][2] * 255.0 * scale) as u8;

        file.write_all(format!("{r} {g} {b}\n").as_ref()).unwrap();
    }
}
