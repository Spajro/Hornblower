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

pub struct Line {
    pub first: Point,
    pub second: Point,
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

    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }
}

impl Paintable for Point {
    fn paint(self, buffer: &mut Buffer) {
        println!("{} {}",self.x,self.y);
        buffer.buffer[self.x + self.y * buffer.width] = Self::from_u8_rgb(128,0,0);
    }
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector {
            x,
            y,
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
        let m = (distance.y as f32) / (distance.x as f32);
        let lower;
        let greater;
        if self.first.x < self.second.x {
            greater = self.second;
            lower = self.first;
        } else {
            greater = self.first;
            lower = self.second;
        }
        println!("M {}", m);
        for x in lower.x..greater.x + 1 {
            let y = (m * ((x - lower.x) as f32) + (lower.y as f32)) as usize;
            Point::new(x, y).paint(buffer);
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