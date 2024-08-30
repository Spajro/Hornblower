use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::shapes::rectangle::Rectangle;
use crate::gui::click::{Click, ClickHandler};

pub struct Throttle {
    pub center: Point,
    pub height: u32,
    pub width: u32,
    pub percent: f32,
    pub color: Color,
}

impl Throttle {
    pub fn new(center: Point, height: u32, width: u32, color: Color) -> Self {
        Self {
            center,
            height,
            width,
            percent: 0.5,
            color,
        }
    }
}

impl ClickHandler for Throttle {
    fn handle_click(&mut self, click: &Click) {
        if click.x < self.center.x as u32 - (self.width / 2) ||
            click.x > self.center.x as u32 + (self.width / 2) ||
            click.y < self.center.y as u32 - (self.height / 2) ||
            click.y > self.center.y as u32 + (self.height / 2)
        {
            return;
        }
        self.percent = -2.0 * (click.y as f32 - self.center.y as f32) / self.height as f32;
    }
}

impl Paintable for Throttle {
    fn paint(&self, buffer: &mut Buffer) {
        let rectangle = Rectangle::new(self.center, Normalized::new(-0.1, 0.0), self.width, self.height,self.color);
        let left_half = (rectangle.first.as_vector() + rectangle.fourth.as_vector()) / 2;
        let right_half = (rectangle.second.as_vector() + rectangle.third.as_vector()) / 2;
        let left_point_half = Point::from_vector(left_half);
        let right_point_half = Point::from_vector(right_half);
        if left_point_half.is_err() || right_point_half.is_err() {
            return;
        }

        let left_state = left_half + (rectangle.first.as_vector() - left_half) * self.percent;
        let right_state = right_half + (rectangle.second.as_vector() - right_half) * self.percent;
        let left_point_state = Point::from_vector(left_state);
        let right_point_state = Point::from_vector(right_state);
        if left_point_state.is_err() || right_point_state.is_err() {
            return;
        }

        rectangle.paint(buffer);
        Line::new(left_point_half.unwrap(), right_point_half.unwrap(),self.color).paint(buffer);
        Line::new(left_point_state.unwrap(), right_point_state.unwrap(),self.color).paint(buffer);
    }
}