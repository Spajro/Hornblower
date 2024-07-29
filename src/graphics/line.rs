use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::point::Point;

pub struct Line {
    pub first: Point,
    pub second: Point,
}impl Line {
    pub fn new(first: Point, second: Point) -> Self {
        Line {
            first,
            second,
        }
    }
}

impl Paintable for Line {
    fn paint(self, buffer: &mut Buffer) {
        let distance = self.second.as_vector() - self.first.as_vector();
        let lower;
        let greater;

        if distance.x == 0 {
            if self.first.y < self.second.y {
                greater = self.second;
                lower = self.first;
            } else {
                greater = self.first;
                lower = self.second;
            }
            for y in lower.y..greater.y + 1 {
                Point::new(lower.x, y).paint(buffer);
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
                let y = (m * ((x - lower.x) as f32) + (lower.y as f32)) as usize;
                Point::new(x, y).paint(buffer);
            }
        }
    }
}