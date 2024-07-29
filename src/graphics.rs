use std::cmp::PartialEq;
use std::ops;
use std::ops::AddAssign;

pub struct Buffer {
    width: usize,
    height: usize,
    pub buffer: Vec<u32>,
}

pub trait Paintable {
    fn paint(self, buffer: &mut Buffer);
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct Normalized {
    pub x: f32,
    pub y: f32,
}

pub struct Line {
    pub first: Point,
    pub second: Point,
}

pub struct Triangle {
    pub first: Point,
    pub second: Point,
    pub third: Point,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point {
            x,
            y,
        }
    }

    fn as_vector(self) -> Vector {
        Vector {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    fn from_vector(vector: Vector) -> Result<Self, &'static str> {
        if vector.x < 0 || vector.y < 0 {
            Err("Point cant have negative coordinates ")
        } else {
            Ok(Point {
                x: vector.x as usize,
                y: vector.y as usize,
            })
        }
    }

    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }
}

impl Paintable for Point {
    fn paint(self, buffer: &mut Buffer) {
        if self.x < buffer.width && self.y < buffer.height {
            buffer.buffer[self.x + (self.y - 1) * buffer.width] = Self::from_u8_rgb(128, 0, 0);
        }
    }
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

impl Line {
    pub fn new(first: Point, second: Point) -> Self {
        Line {
            first,
            second,
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

impl Paintable for Line {
    fn paint(self, buffer: &mut Buffer) {
        let distance = self.second.as_vector() - self.first.as_vector();
        let lower;
        let greater;

        if distance.x == 0 {
            if self.first.y < self.second.y {
                greater = self.second;
                lower = self.first;
            } else {
                greater = self.first;
                lower = self.second;
            }
            for y in lower.y..greater.y + 1 {
                Point::new(lower.x, y).paint(buffer);
            }
        } else {
            let m = (distance.y as f32) / (distance.x as f32);
            if self.first.x < self.second.x {
                greater = self.second;
                lower = self.first;
            } else {
                greater = self.first;
                lower = self.second;
            }
            for x in lower.x..greater.x + 1 {
                let y = (m * ((x - lower.x) as f32) + (lower.y as f32)) as usize;
                Point::new(x, y).paint(buffer);
            }
        }
    }
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }
}

impl Paintable for Triangle {
    fn paint(self, buffer: &mut Buffer) {
        Line::new(self.first, self.second).paint(buffer);
        Line::new(self.second, self.third).paint(buffer);
        Line::new(self.first, self.third).paint(buffer);
    }
}

impl Triangle {
    pub fn new(first: Point, second: Point, third: Point) -> Self {
        Triangle {
            first,
            second,
            third,
        }
    }

    pub fn equilateral(center: Point, direction: Normalized, length: u32) -> Self {
        let radius = ((length as f32) / 3_f32.sqrt()) as i32;
        let tv = center.as_vector();
        let v1 = tv + (direction * radius);
        let v2 = tv + (direction * radius).rotate(3.14 * 2.0 / 3.0);
        let v3 = tv + (direction * radius).rotate(3.14 * 4.0 / 3.0);

        let first = Point::from_vector(v1).unwrap();
        let second = Point::from_vector(v2).unwrap();
        let third = Point::from_vector(v3).unwrap();
        Triangle {
            first,
            second,
            third,
        }
    }
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