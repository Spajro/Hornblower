use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::square::Square;
use crate::gui::click::{Click, ClickHandler};

pub struct TickButton {
    pub center: Point,
    pub size: u32,
    pub direction: Normalized,
    click: bool,
}

impl TickButton {
    pub fn new(center: Point, size: u32) -> Self {
        TickButton {
            center,
            size,
            direction: Normalized::new(0.0, -1.0),
            click: false,
        }
    }

    pub fn clicked(&self) -> bool {
        self.click
    }
}

impl Paintable for TickButton {
    fn paint(&self, buffer: &mut Buffer) {
        Square::new(self.center, Normalized::new(0.0, 1.0), 2 * self.size).paint(buffer);
        let inside = Square::new(self.center, Normalized::new(0.0, -1.0), (1.8 * self.size as f32) as u32);
        inside.paint(buffer);

        if self.click {
            let center = Line::new(inside.second, inside.third).center();
            Line::new(inside.first, center).paint(buffer);
            Line::new(inside.fourth, center).paint(buffer);
        }
    }
}

impl ClickHandler for TickButton {
    fn handle_click(&mut self, click: &Click) {
        if click.x < self.center.x as u32 - (self.size / 2) ||
            click.x > self.center.x as u32 + (self.size / 2) ||
            click.y < self.center.y as u32 - (self.size / 2) ||
            click.y > self.center.y as u32 + (self.size / 2)
        {
            return;
        }
        self.click = !self.click
    }
}