use std::ops;
use crate::physics::vector2d::Vector2D;

pub struct Normalized2D {
    x: f64,
    y: f64,
}

impl Normalized2D {
    pub fn new(x: f64, y: f64) -> Self {
        Normalized2D { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
}

impl ops::Mul<i64> for &Normalized2D {
    type Output = Vector2D;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector2D {
            x: (self.x * (rhs as f64)) as i64,
            y: (self.y * (rhs as f64)) as i64,
        }
    }
}