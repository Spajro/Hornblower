use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::circle_with_radius::CircleWithRadius;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::square::Square;
use crate::gui::click::Click;

pub struct Compass {
    pub center: Point,
    pub size: u32,
    pub direction: Normalized,
}

impl Compass {
    pub fn new(center: Point, size: u32) -> Self {
        Compass {
            center,
            size,
            direction: Normalized::new(0.0, -1.0),
        }
    }

    pub fn update(&mut self, direction: Normalized) {
        self.direction = direction;
    }

    pub fn handle_click(&mut self, click: &Click) {
        if click.x < self.center.x as u32 - (self.size / 2) ||
            click.x > self.center.x as u32 + (self.size / 2) ||
            click.y < self.center.y as u32 - (self.size / 2) ||
            click.y > self.center.y as u32 + (self.size / 2)
        {
            return;
        }
        self.direction = (click.as_vector() - self.center.as_vector()).normalize();
    }
}

impl Paintable for Compass {
    fn paint(&self, buffer: &mut Buffer) {
        Square::new(self.center, Normalized::new(0.0, -1.0), 2*self.size).paint(buffer);
        CircleWithRadius::new(self.center, self.size/2, self.direction).paint(buffer);
    }
}