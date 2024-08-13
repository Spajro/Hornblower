use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::gui::button::Button;
use crate::gui::click::{Click, ClickHandler};

pub struct PlusMinus {
    plus: Button,
    minus: Button,
    value: i32,
}

impl PlusMinus {
    pub fn new(center: Point, size: u32, direction: Normalized, initial_value: i32) -> Self {
        let half_size = (size / 2) as usize;
        let left_center = Point::new(center.x - half_size, center.y);
        let right_center = Point::new(center.x + half_size, center.y);
        PlusMinus {
            plus: Button::with_plus(left_center, half_size as u32),
            minus: Button::with_minus(right_center, half_size as u32),
            value: initial_value,
        }
    }

    pub fn get_value(&self)->i32{
        self.value
    }
}

impl Paintable for PlusMinus {
    fn paint(&self, buffer: &mut Buffer) {
        self.plus.paint(buffer);
        self.minus.paint(buffer);
    }
}

impl ClickHandler for PlusMinus {
    fn handle_click(&mut self, click: &Click) {
        self.plus.handle_click(click);
        self.minus.handle_click(click);
        if self.plus.clicked() {
            self.value += 1;
        }
        if self.minus.clicked() {
            self.value -= 1;
        }
    }
}