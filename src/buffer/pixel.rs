use crate::Color;

pub struct Pixel {
    pub color: Color,
}

impl Pixel {
    pub fn default(&mut self) -> Pixel {
        Pixel {
            color: Color { r: 0, g: 0, b: 0 },
        }
    }
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {
            color: Color { r, g, b },
        }
    }
}
