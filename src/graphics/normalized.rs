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