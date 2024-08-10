use crate::graphics::vector::Vector;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
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
                x: vector.x as usize,
                y: vector.y as usize,
            })
        }
    }
}