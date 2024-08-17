use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::gui::button::Button;
use crate::gui::click::{Click, ClickHandler};
use crate::gui::compass::Compass;
use crate::gui::plus_minus::PlusMinus;
use crate::gui::throttle::Throttle;
use crate::physics::engine::Engine;
use crate::physics::normalized2d::Normalized2D;

pub struct Interface {
    throttle: Throttle,
    compass: Compass,
    zoom: PlusMinus,
    fire_compass: Compass,
    fire_button: Button,
}

impl Interface {
    pub fn new(height: u32, width: u32, scale: u32) -> Self {
        let size = 40;
        let half_size = size / 2;
        let direction = Normalized::new(1.0, 0.0);
        Interface {
            throttle: Throttle::new(Point::new(half_size, height - size), 2 * size, size),
            compass: Compass::new(Point::new(2 * size, height - size), 2 * size),
            zoom: PlusMinus::new(Point::new(size, half_size), size, direction, scale as i32),
            fire_compass: Compass::new(Point::new(width - size, height - size), 2 * size),
            fire_button: Button::new(Point::new(width - 3 * size, height - size), size),
        }
    }

    fn normalized_to_normalized2d(normalized: Normalized) -> Normalized2D {
        Normalized2D::new(normalized.x as f64, normalized.y as f64)
    }

    pub fn handle_click(&mut self, click: &Click, engine: &mut Engine, id: u32) {
        self.zoom.handle_click(click);
        self.compass.handle_click(click);
        self.throttle.handle_click(click);
        self.fire_compass.handle_click(click);
        self.fire_button.handle_click(click);
        engine.accelerate(id, Self::normalized_to_normalized2d(self.compass.direction) * ((40.0 * self.throttle.percent) as i64));
        engine.set_scale(self.zoom.get_value() as u32);
    }
}

impl Paintable for Interface {
    fn paint(&self, buffer: &mut Buffer) {
        self.throttle.paint(buffer);
        self.compass.paint(buffer);
        self.zoom.paint(buffer);
        self.fire_compass.paint(buffer);
        self.fire_button.paint(buffer);
    }
}
