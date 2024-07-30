use std::fmt::{Display, Formatter};
use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::point::Point;
use crate::graphics::triangle::Triangle;
use crate::graphics::vector::Vector;
use crate::status::Status;

pub struct Engine {
    pub objects: Vec<Status>,
    tick_rate: u32,
    scale: u32,
}

impl Engine {
    pub fn update(&mut self) {
        self.objects.iter_mut().for_each(|o| o.update(self.tick_rate))
    }
    pub fn register(&mut self, object: Status) {
        self.objects.push(object)
    }

    pub fn new(tick_rate: u32, scale: u32) -> Self {
        Engine {
            objects: vec![],
            tick_rate,
            scale,
        }
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(
            for o in &self.objects {
                write!(f, "{}\n", o);
            })
    }
}

impl Paintable for Engine {
    fn paint(&self, buffer: &mut Buffer) {
        self.objects.iter()
            .map(|status| Triangle::equilateral(
                Point::new(
                    (status.position.x as f32 / self.scale as f32) as usize,
                    (status.position.y as f32 / self.scale as f32) as usize),
                Vector::new(status.speed.x as i32, status.speed.y as i32).normalize(),
                10))
            .for_each(|triangle| triangle.paint(buffer));
    }
}