use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;

pub struct Square {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
    direction: Normalized,
}

impl Square {
    pub fn new(center: Point, direction: Normalized, length: u32) -> Self {
        let half_length = (length / 2) as usize;
        Square {
            x1: center.x - half_length,
            x2: center.x + half_length,
            y1: center.y - half_length,
            y2: center.y + half_length,
            direction,
        }
    }

    fn rotate_by_direction(&self, point: Point) -> Point {
        Point::from_vector(point.as_vector().rotate(self.direction.angle(&Normalized::new(1.0, 0.0)))).unwrap()
    }

    pub fn get_first(&self) -> Point {
        self.rotate_by_direction(Point::new(self.x1, self.y1))
    }
    pub fn get_second(&self) -> Point {
        self.rotate_by_direction(Point::new(self.x2, self.y1))
    }
    pub fn get_third(&self) -> Point {
        self.rotate_by_direction(Point::new(self.x2, self.y2))
    }
    pub fn get_fourth(&self) -> Point {
        self.rotate_by_direction(Point::new(self.x1, self.y2))
    }
}

impl Paintable for Square {
    fn paint(&self, buffer: &mut Buffer) {
        Line::new(self.get_first(), self.get_second()).paint(buffer);
        Line::new(self.get_second(), self.get_third()).paint(buffer);
        Line::new(self.get_third(), self.get_fourth()).paint(buffer);
        Line::new(self.get_fourth(), self.get_first()).paint(buffer);
    }
}