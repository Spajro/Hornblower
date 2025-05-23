use std::f32::consts::PI;
use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::point::Point;
use crate::graphics::vector::Vector;

pub struct Circle {
    pub center: Point,
    pub radius: u32,
    pub color: Color,
}

impl Circle {
    pub fn new(center: Point, radius: u32, color: Color) -> Self {
        Circle {
            center,
            radius,
            color,
        }
    }
}

impl Paintable for Circle {
    fn paint(&self, buffer: &mut Buffer) {
        let vector = Vector::new(self.radius as i32, 0);
        let cnt = self.center.as_vector();
        for i in 0..720 {
            let angle = PI * 4.0 * i as f32 / 720.0;
            buffer.paint_pixel(cnt + vector.rotate(angle),&self.color)
        }
    }
}