use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::point::Point;
use crate::graphics::triangle::Triangle;
use crate::graphics::vector::Vector;
use crate::status::Status;

pub struct Engine {
    pub status_map: HashMap<u32, Status>,
    tick_rate: u32,
    scale: u32,
}

impl Engine {
    pub fn update(&mut self) {
        self.status_map.iter_mut().for_each(|(id, status)| status.update(self.tick_rate));
    }
    pub fn register(&mut self, id: u32, object: Status) {
        self.status_map.insert(id, object);
    }

    pub fn new(tick_rate: u32, scale: u32) -> Self {
        Engine {
            status_map: HashMap::new(),
            tick_rate,
            scale,
        }
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(
            for (id, status) in &self.status_map {
                write!(f, "{} : {}\n", id, status);
            })
    }
}

impl Paintable for Engine {
    fn paint(&self, buffer: &mut Buffer) {
        let width = buffer.width;
        let height = buffer.height;
        self.status_map.iter()
            .map(|(id, status)| Triangle::equilateral(
                Point::new(
                    (status.position.x as f32 / self.scale as f32) as usize + (width / 2),
                    (status.position.y as f32 / self.scale as f32) as usize + (height / 2)),
                Vector::new(status.speed.x as i32, status.speed.y as i32).normalize(),
                10))
            .for_each(|triangle| triangle.paint(buffer));
    }
}