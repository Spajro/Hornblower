use std::ops;
use std::ops::{AddAssign, Mul};
use std::process::Output;
use crate::graphics::normalized::Normalized;

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector {
            x,
            y,
        }
    }

    pub fn length(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    pub fn normalize(self) -> Normalized {
        Normalized {
            x: self.x as f32 / self.length() as f32,
            y: self.y as f32 / self.length() as f32,
        }
    }

    pub fn rotate(self, angle: f32) -> Vector {
        Vector {
            x: ((self.x as f32) * angle.cos() - (self.y as f32) * angle.sin()) as i32,
            y: ((self.x as f32) * angle.sin() + (self.y as f32) * angle.cos()) as i32,
        }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Div<i32> for Vector {
    type Output = Vector;

    fn div(self, rhs: i32) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y /rhs,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
       Vector{
           x: (self.x as f32 * rhs) as i32,
           y: (self.y as f32 * rhs) as i32,
       }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}