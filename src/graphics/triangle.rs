use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;

pub struct Triangle {
    pub first: Point,
    pub second: Point,
    pub third: Point,
}

impl Paintable for Triangle {
    fn paint(self, buffer: &mut Buffer) {
        Line::new(self.first, self.second).paint(buffer);
        Line::new(self.second, self.third).paint(buffer);
        Line::new(self.first, self.third).paint(buffer);
    }
}

impl Triangle {
    pub fn new(first: Point, second: Point, third: Point) -> Self {
        Triangle {
            first,
            second,
            third,
        }
    }

    pub fn equilateral(center: Point, direction: Normalized, length: u32) -> Self {
        let radius = ((length as f32) / 3_f32.sqrt()) as i32;
        let tv = center.as_vector();
        let v1 = tv + (direction * radius);
        let v2 = tv + (direction * radius).rotate(3.14 * 2.0 / 3.0);
        let v3 = tv + (direction * radius).rotate(3.14 * 4.0 / 3.0);

        let first = Point::from_vector(v1).unwrap();
        let second = Point::from_vector(v2).unwrap();
        let third = Point::from_vector(v3).unwrap();
        Triangle {
            first,
            second,
            third,
        }
    }
}