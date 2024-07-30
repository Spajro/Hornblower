use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::vector::Vector;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point {
            x,
            y,
        }
    }

    pub fn as_vector(self) -> Vector {
        Vector {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn from_vector(vector: Vector) -> Result<Self, &'static str> {
        if vector.x < 0 || vector.y < 0 {
            Err("Point cant have negative coordinates ")
        } else {
            Ok(Point {
                x: vector.x as usize,
                y: vector.y as usize,
            })
        }
    }

    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }
}

impl Paintable for Point {
    fn paint(&self, buffer: &mut Buffer) {
        if self.x < buffer.width && self.y < buffer.height {
            buffer.buffer[self.x + (self.y - 1) * buffer.width] = Self::from_u8_rgb(128, 0, 0);
        }
    }
}