use std::ops;
use crate::physics::float_vector2d::FloatVector2D;
use crate::physics::vector2d::Vector2D;

pub struct Normalized2D {
    x: f64,
    y: f64,
}

impl Normalized2D {
    pub fn from(vector2d: &Vector2D) -> Self {
        Normalized2D {
            x: vector2d.x as f64 / vector2d.length() as f64,
            y: vector2d.y as f64 / vector2d.length() as f64,
        }
    }

    pub fn from_float(vector2d: &FloatVector2D) -> Self {
        Normalized2D {
            x: vector2d.x / vector2d.length(),
            y: vector2d.y / vector2d.length(),
        }
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