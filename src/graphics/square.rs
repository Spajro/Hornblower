use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;

pub struct Square {
    pub first: Point,
    pub second: Point,
    pub third: Point,
    pub fourth: Point,
}

impl Square {
    pub fn new(center: Point, direction: Normalized, length: u32) -> Self {
        let distance = ((length as f32) / (2.0_f32 * 2.0_f32.sqrt())) as i32;
        let tv = center.as_vector();
        let vector = direction * distance;
        let v1 = tv + vector.rotate(std::f32::consts::PI * 1.0 / 4.0);
        let v2 = tv + vector.rotate(std::f32::consts::PI * 3.0 / 4.0);
        let v3 = tv + vector.rotate(std::f32::consts::PI * 5.0 / 4.0);
        let v4 = tv + vector.rotate(std::f32::consts::PI * 7.0 / 4.0);
        let first = Point::from_vector(v1).unwrap();
        let second = Point::from_vector(v2).unwrap();
        let third = Point::from_vector(v3).unwrap();
        let fourth = Point::from_vector(v4).unwrap();
        Square {
            first,
            second,
            third,
            fourth,
        }
    }
}

impl Paintable for Square{
    fn paint(self, buffer: &mut Buffer) {
        Line::new(self.first, self.second).paint(buffer);
        Line::new(self.second, self.third).paint(buffer);
        Line::new(self.third, self.fourth).paint(buffer);
        Line::new(self.fourth, self.first).paint(buffer);
    }
}