use std::fmt::Display;
use std::ops;
use crate::engine::math::float_vector2d::FloatVector2D;
use crate::engine::math::normalized2d::Normalized2D;

#[derive(Copy, Clone)]
pub struct Vector2D {
    pub x: i64,
    pub y: i64,
}

impl Vector2D {
    pub fn new(x: i64, y: i64) -> Self {
        Vector2D { x, y }
    }

    pub fn length(&self) -> u64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt() as u64
    }

    pub fn normalize(&self) -> Normalized2D {
        Normalized2D::from(self)
    }

    pub fn distance(&self, other: &Vector2D) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    pub fn add_assign_with_carry(&mut self, rhs: FloatVector2D, carry: &FloatVector2D) -> FloatVector2D {
        let t = rhs + carry;
        self.x += t.x.floor() as i64;
        self.y += t.y.floor() as i64;
        FloatVector2D::new(t.x - t.x.floor(), t.y - t.y.floor())
    }
}

impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul<i64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<f64> for Vector2D {
    type Output = FloatVector2D;

    fn mul(self, rhs: f64) -> Self::Output {
        FloatVector2D {
            x: self.x as f64 * rhs,
            y: self.y as f64 * rhs,
        }
    }
}

impl ops::Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{},{}>", self.x, self.y)
    }
}