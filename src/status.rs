use std::fmt::{Display, Formatter};
use crate::vector2d::Vector2D;

pub struct Status {
    pub position: Vector2D,
    pub speed: Vector2D,
    pub acceleration: Vector2D,
}

impl Status {
    pub(crate) fn new() -> Self {
        Status {
            position: Vector2D::new(0, 0),
            speed: Vector2D::new(0, 0),
            acceleration: Vector2D::new(0, 0),
        }
    }
    pub fn with_position(position: Vector2D) -> Self {
        Status {
            position: position,
            speed: Vector2D::new(0, 0),
            acceleration: Vector2D::new(0, 0),
        }
    }

    pub fn accelerate(&mut self, acceleration: Vector2D) {
        self.acceleration = acceleration;
    }

    pub fn update(&mut self) {
        self.position += self.speed;
        self.speed += self.acceleration;
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<p={}\ns={}\na={}>", self.position, self.speed, self.acceleration)
    }
}
