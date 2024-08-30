use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;

pub struct Triangle {
    pub first: Point,
    pub second: Point,
    pub third: Point,
    pub color: Color,
}

impl Paintable for Triangle {
    fn paint(&self, buffer: &mut Buffer) {
        Line::new(self.first, self.second, self.color).paint(buffer);
        Line::new(self.second, self.third, self.color).paint(buffer);
        Line::new(self.first, self.third, self.color).paint(buffer);
    }
}

impl Triangle {
    pub fn new(first: Point, second: Point, third: Point, color: Color) -> Self {
        Triangle {
            first,
            second,
            third,
            color,
        }
    }

    pub fn equilateral(center: Point, direction: Normalized, length: u32, color: Color) -> Result<Self, &'static str> {
        let radius = ((length as f32) / 3_f32.sqrt()) as i32;
        let v = center.as_vector();
        let p1 = Point::from_vector(v + (direction * radius));
        let p2 = Point::from_vector(v + (direction * radius).rotate(3.14 * 2.0 / 3.0));
        let p3 = Point::from_vector(v + (direction * radius).rotate(3.14 * 4.0 / 3.0));

        return if p1.is_err() {
            Err(p1.err().unwrap())
        } else if p2.is_err() {
            Err(p2.err().unwrap())
        } else if p3.is_err() {
            Err(p3.err().unwrap())
        } else {
            Ok(Triangle {
                first: p1.unwrap(),
                second: p2.unwrap(),
                third: p3.unwrap(),
                color,
            })
        };
    }
}