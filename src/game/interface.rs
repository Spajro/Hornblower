use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::gui::button::Button;
use crate::gui::click::{Click, ClickHandler};
use crate::gui::compass::Compass;
use crate::gui::plus_minus::PlusMinus;
use crate::gui::throttle::Throttle;
use crate::physics::engine::Event;
use crate::physics::id::ID;
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
        let color = Color::RED;
        Interface {
            throttle: Throttle::new(Point::new(half_size, height - size), 2 * size, size, color),
            compass: Compass::new(Point::new(2 * size, height - size), 2 * size, color),
            zoom: PlusMinus::new(Point::new(size, half_size), size, direction, scale as i32, color),
            fire_compass: Compass::new(Point::new(width - size, height - size), 2 * size, color),
            fire_button: Button::new(Point::new(width - 3 * size, height - size), size, color),
        }
    }

    fn normalized_to_normalized2d(normalized: Normalized) -> Normalized2D {
        Normalized2D::new(normalized.x as f64, normalized.y as f64)
    }

    pub fn handle_click(&mut self, click: &Click, id: ID) -> Vec<Event> {
        self.zoom.handle_click(click);
        self.compass.handle_click(click);
        self.throttle.handle_click(click);
        self.fire_compass.handle_click(click);
        self.fire_button.handle_click(click);

        let mut result = vec![];
        result.push(Event::Accelerate(id, Self::normalized_to_normalized2d(self.compass.direction), self.throttle.percent));
        result.push(Event::Scale(self.zoom.get_value() as u32));
        if self.fire_button.clicked() {
            result.push(Event::Fire(id, Self::normalized_to_normalized2d(self.fire_compass.direction)));
        }

        result
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
