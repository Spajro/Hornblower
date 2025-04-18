use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::point::Point;
use crate::graphics::vector::Vector;

pub struct Line {
    pub first: Point,
    pub second: Point,
    pub color: Color,
}

impl Line {
    pub fn new(first: Point, second: Point, color: Color) -> Self {
        Line {
            first,
            second,
            color,
        }
    }

    pub fn center(&self) -> Point {
        Point::new(
            (self.first.x + self.second.x) / 2,
            (self.first.y + self.second.y) / 2,
        )
    }
}

impl Paintable for Line {
    fn paint(&self, buffer: &mut Buffer) {
        let distance = self.second.as_vector() - self.first.as_vector();
        let lower;
        let greater;

        if distance.x.abs() <= 1 {
            if self.first.y < self.second.y {
                greater = self.second;
                lower = self.first;
            } else {
                greater = self.first;
                lower = self.second;
            }
            for y in lower.y..greater.y + 1 {
                buffer.paint_pixel(Vector::new(lower.x as i32, y as i32), &self.color);
            }
        } else {
            let m = (distance.y as f32) / (distance.x as f32);
            if self.first.x < self.second.x {
                greater = self.second;
                lower = self.first;
            } else {
                greater = self.first;
                lower = self.second;
            }
            for x in lower.x..greater.x + 1 {
                let y = (m * ((x - lower.x) as f32) + (lower.y as f32)) as i32;
                buffer.paint_pixel(Vector::new(x as i32, y), &self.color);
            }
        }
    }
}