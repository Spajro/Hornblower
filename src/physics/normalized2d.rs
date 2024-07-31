use std::fmt::Display;
use std::ops;
use crate::physics::vector2d::Vector2D;

pub struct Normalized2D {
    pub x: f64,
    pub y: f64,
}

pub struct PerpendicularResult {
    pub left: Normalized2D,
    pub right: Normalized2D,
}

impl Normalized2D {
    pub fn new(x: f64, y: f64) -> Self {
        Normalized2D { x, y }
    }
    pub fn perpendicular(&self) -> PerpendicularResult {
        let first = Normalized2D::new(self.y, -self.x);
        let second = Normalized2D::new(-self.y, self.x);
        if first.x < second.x {
            PerpendicularResult { left: second, right: first }
        } else {
            PerpendicularResult { left: first, right: second }
        }
    }
}

impl ops::Mul<i64> for Normalized2D {
    type Output = Vector2D;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector2D {
            x: (self.x * (rhs as f64)) as i64,
            y: (self.y * (rhs as f64)) as i64,
        }
    }
}

impl Display for Normalized2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{}>", self.x, self.y)
    }
}