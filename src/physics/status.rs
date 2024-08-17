use std::fmt::{Display, Formatter};
use crate::physics::vector2d::Vector2D;

pub struct Status {
    position: Vector2D,
    speed: Vector2D,
    acceleration: Vector2D,
}

impl Status {
    pub fn new() -> Self {
        Status {
            position: Vector2D::new(0, 0),
            speed: Vector2D::new(0, 0),
            acceleration: Vector2D::new(0, 0),
        }
    }
    pub fn with_position(position: Vector2D) -> Self {
        Status {
            position,
            speed: Vector2D::new(0, 0),
            acceleration: Vector2D::new(0, 0),
        }
    }

    pub fn with_position_and_speed(position: Vector2D, speed: Vector2D) -> Self {
        Status {
            position,
            speed,
            acceleration: Vector2D::new(0, 0),
        }
    }

    pub fn accelerate(&mut self, acceleration: Vector2D) {
        self.acceleration = acceleration;
    }

    pub fn update(&mut self, tick_rate: u32) {
        let percent = 1.0 / (tick_rate as f32);
        self.position += self.speed * percent;
        self.speed += self.acceleration * percent;
    }

    pub fn position(&self) -> Vector2D {
        self.position
    }

    pub fn acceleration(&self) -> Vector2D {
        self.acceleration
    }

    pub fn speed(&self) -> Vector2D {
        self.speed
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<p={}\ns={}\na={}>", self.position, self.speed, self.acceleration)
    }
}
