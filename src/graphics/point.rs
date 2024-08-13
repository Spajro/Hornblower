use crate::graphics::vector::Vector;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Point {
            x,
            y,
        }
    }

    pub fn as_vector(self) -> Vector {
        Vector {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn from_vector(vector: Vector) -> Result<Self, &'static str> {
        if vector.x < 0 || vector.y < 0 {
            Err("Point cant have negative coordinates ")
        } else {
            Ok(Point {
                x: vector.x as u32,
                y: vector.y as u32,
            })
        }
    }
}