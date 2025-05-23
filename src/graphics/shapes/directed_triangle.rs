use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::shapes::triangle::Triangle;

pub struct DirectedTriangle {
    pub triangle: Triangle,
    pub color: Color,
}

impl DirectedTriangle {
    pub fn equilateral(center: Point, direction: Normalized, length: u32,color: Color) -> Result<Self, &'static str> {
        Triangle::equilateral(center, direction, length,color).map(|triangle| DirectedTriangle { triangle,color })
    }
}

impl Paintable for DirectedTriangle {
    fn paint(&self, buffer: &mut Buffer) {
        self.triangle.paint(buffer);
        let point = Point::new(
            (self.triangle.second.x + self.triangle.third.x) / 2,
            (self.triangle.second.y + self.triangle.third.y) / 2,
        );
        Line::new(self.triangle.first, point,self.color).paint(buffer);
    }
}