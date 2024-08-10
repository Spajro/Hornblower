use crate::graphics::vector::Vector;

pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

pub trait Paintable {
    fn paint(&self, buffer: &mut Buffer);
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn paint_pixel(&mut self, position: Vector) {
        if position.x < self.width as i32 && position.y < self.height as i32 {
            self.buffer[(position.x as usize) + (position.y as usize) * self.width] = Self::from_u8_rgb(128, 0, 0);
        }
    }
}