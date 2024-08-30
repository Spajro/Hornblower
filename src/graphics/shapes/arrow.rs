use std::f32::consts::PI;
use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::line::Line;
use crate::graphics::point::Point;
use crate::graphics::vector::Vector;

pub struct Arrow {
    start: Point,
    end: Point,
    color: Color,
}

impl Arrow {
    pub fn new(start: Point, end: Point, color: Color) -> Self {
        Self { start, end, color }
    }

    fn as_vector(&self) -> Vector {
        Vector::new(
            self.end.x as i32 - self.start.x as i32,
            self.end.y as i32 - self.start.x as i32,
        )
    }

    pub fn length(&self) -> f32 {
        self.as_vector().length()
    }
}

impl Paintable for Arrow {
    fn paint(&self, buffer: &mut Buffer) {
        Line::new(self.start,self.end,self.color).paint(buffer);
        let rev=self.as_vector()*-0.2;
        let left_point=Point::from_vector(rev.rotate(PI/8.0)+self.end.as_vector()).unwrap();
        let right_point=Point::from_vector(rev.rotate(15.0*PI/8.0)+self.end.as_vector()).unwrap();
        Line::new(left_point,self.end,self.color).paint(buffer);
        Line::new(right_point,self.end,self.color).paint(buffer);
    }
}