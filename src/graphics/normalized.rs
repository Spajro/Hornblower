use std::ops;
use crate::graphics::vector::Vector;

#[derive(Clone, Copy)]
pub struct Normalized {
    pub x: f32,
    pub y: f32,
}

impl Normalized {
    pub fn new(x: f32, y: f32) -> Self {
        Normalized {
            x,
            y,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Normalized) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn angle(&self, other: &Normalized) -> f32 {
        (self.dot(other) / (self.length() * other.length())).asin()
    }
}

impl ops::Mul<i32> for Normalized {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector {
            x: (self.x * (rhs as f32)) as i32,
            y: (self.y * (rhs as f32)) as i32,
        }
    }
}