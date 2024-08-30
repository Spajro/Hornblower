use std::ops;

pub struct FloatVector2D {
    pub x: f64,
    pub y: f64,
}

impl FloatVector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl ops::Mul<f64> for FloatVector2D {
    type Output = FloatVector2D;

    fn mul(self, rhs: f64) -> Self::Output {
        FloatVector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Add<&FloatVector2D> for FloatVector2D {
    type Output = FloatVector2D;

    fn add(self, rhs: &Self) -> Self::Output {
        FloatVector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}