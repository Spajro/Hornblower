use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::shapes::square::Square;
use crate::gui::click::{Click, ClickHandler};

pub struct TickButton {
    pub center: Point,
    pub size: u32,
    pub direction: Normalized,
    click: bool,
    color: Color,
}

impl TickButton {
    pub fn new(center: Point, size: u32, color: Color) -> Self {
        TickButton {
            center,
            size,
            direction: Normalized::new(0.0, -1.0),
            click: false,
            color,
        }
    }

    pub fn clicked(&self) -> bool {
        self.click
    }
}

impl Paintable for TickButton {
    fn paint(&self, buffer: &mut Buffer) {
        Square::new(self.center, Normalized::new(0.0, 1.0), 2 * self.size, self.color).paint(buffer);
        let inside = Square::new(self.center, Normalized::new(0.0, 1.0), (1.8 * self.size as f32) as u32, self.color);
        inside.paint(buffer);

        if self.click {
            let center = Line::new(inside.get_second(), inside.get_third(),self.color).center();
            Line::new(inside.get_first(), center, self.color).paint(buffer);
            Line::new(inside.get_fourth(), center, self.color).paint(buffer);
        }
    }
}

impl ClickHandler for TickButton {
    fn handle_click(&mut self, click: &Click) {
        if click.x < self.center.x - (self.size / 2) ||
            click.x > self.center.x + (self.size / 2) ||
            click.y < self.center.y - (self.size / 2) ||
            click.y > self.center.y + (self.size / 2)
        {
            return;
        }
        self.click = !self.click
    }
}