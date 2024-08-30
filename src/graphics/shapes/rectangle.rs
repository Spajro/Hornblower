use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::vector::Vector;

pub struct Rectangle {
    pub first: Point,
    pub second: Point,
    pub third: Point,
    pub fourth: Point,
    pub color: Color,
}

impl Rectangle {
    pub fn new(center: Point, direction: Normalized, width: u32, height: u32, color: Color) -> Self {
        let tv = center.as_vector();
        let angle = direction.angle(&Normalized::new(1.0, 0.0));
        let v1 = tv + Vector::new(height as i32 / 2, -(width as i32 / 2)).rotate(angle);
        let v2 = tv + Vector::new(height as i32 / 2, width as i32 / 2).rotate(angle);
        let v3 = tv + Vector::new(-(height as i32 / 2), width as i32 / 2).rotate(angle);
        let v4 = tv + Vector::new(-(height as i32 / 2), -(width as i32 / 2)).rotate(angle);
        let first = Point::from_vector(v1).unwrap();
        let second = Point::from_vector(v2).unwrap();
        let third = Point::from_vector(v3).unwrap();
        let fourth = Point::from_vector(v4).unwrap();
        Rectangle {
            first,
            second,
            third,
            fourth,
            color,
        }
    }
}

impl Paintable for Rectangle {
    fn paint(&self, buffer: &mut Buffer) {
        Line::new(self.first, self.second, self.color).paint(buffer);
        Line::new(self.second, self.third, self.color).paint(buffer);
        Line::new(self.third, self.fourth, self.color).paint(buffer);
        Line::new(self.fourth, self.first, self.color).paint(buffer);
    }
}