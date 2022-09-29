use std::io::Write;

use spyder_math::*;

pub type Colour = Vec3d;

pub trait WritePpm {
    fn write_ppm(&self, file: &mut impl Write, samples: u32);
}

pub trait MultiplyColour {
    fn multiply_colour(self, colour: Colour) -> Self;
}

impl WritePpm for Colour {
    fn write_ppm(&self, file: &mut impl Write, samples: u32) {
        let scale = 1.0 / samples as f64;

        let r = ((self.data[0][0] * scale).sqrt() * 255.0) as u8;
        let g = ((self.data[0][1] * scale).sqrt() * 255.0) as u8;
        let b = ((self.data[0][2] * scale).sqrt() * 255.0) as u8;

        file.write_all(format!("{r} {g} {b}\n").as_ref()).unwrap();
    }
}

impl MultiplyColour for Colour {
    fn multiply_colour(self, colour: Colour) -> Self {
        Self::new(
            self.data[0][0] * colour.data[0][0],
            self.data[0][1] * colour.data[0][1],
            self.data[0][2] * colour.data[0][2],
        )
    }
}
