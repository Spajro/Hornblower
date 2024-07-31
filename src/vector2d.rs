use std::fmt::Display;
use std::ops;
use crate::normalized2d::Normalized2D;

#[derive(Copy, Clone)]
pub struct Vector2D {
    pub x: i64,
    pub y: i64,
}

impl Vector2D {
    pub fn new(x: i64, y: i64) -> Self {
        Vector2D { x, y }
    }

    pub fn length(&self) -> i64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt() as i64
    }

    pub fn normalize(&self) -> Normalized2D {
        Normalized2D {
            x: self.x as f64 / self.length() as f64,
            y: self.y as f64 / self.length() as f64,
        }
    }

    pub fn distance(&self, other: &Vector2D) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
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

impl ops::Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2D {
            x: (self.x as f32 * rhs) as i64,
            y: (self.y as f32 * rhs) as i64,
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