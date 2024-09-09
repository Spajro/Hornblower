use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::shapes::circle_with_radius::CircleWithRadius;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::shapes::square::Square;
use crate::gui::click::{Click, ClickHandler};

pub struct Compass {
    pub center: Point,
    pub size: u32,
    pub direction: Normalized,
    pub color: Color,
}

impl Compass {
    pub fn new(center: Point, size: u32, color: Color) -> Self {
        Compass {
            center,
            size,
            direction: Normalized::new(0.0, -1.0),
            color,
        }
    }

    pub fn update(&mut self, direction: Normalized) {
        self.direction = direction;
    }
}

impl ClickHandler for Compass {
    fn handle_click(&mut self, click: &Click) {
        if click.x < self.center.x - (self.size / 2) ||
            click.x > self.center.x + (self.size / 2) ||
            click.y < self.center.y - (self.size / 2) ||
            click.y > self.center.y + (self.size / 2)
        {
            return;
        }
        self.direction = (click.as_vector() - self.center.as_vector()).normalize().unwrap_or(self.direction);
    }
}

impl Paintable for Compass {
    fn paint(&self, buffer: &mut Buffer) {
        Square::new(self.center, Normalized::new(0.0, 1.0), self.size, self.color).paint(buffer);
        CircleWithRadius::new(self.center, self.size / 2, self.direction, self.color).paint(buffer);
    }
}