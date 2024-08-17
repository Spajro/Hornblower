use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use crate::physics::collider::{CircleCollider2D, Collider};
use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::shapes::directed_triangle::DirectedTriangle;
use crate::graphics::point::Point;
use crate::graphics::vector::Vector;
use crate::physics::id::IdFactory;
use crate::physics::limitations::Limitations;
use crate::physics::status::Status;
use crate::physics::vector2d::Vector2D;

pub struct Engine {
    id_factory: IdFactory,
    status_map: HashMap<u32, Status>,
    collider_map: HashMap<u32, CircleCollider2D>,
    limitations_map: HashMap<u32, Limitations>,
    tick_rate: u32,
    scale: u32,
}

impl Engine {
    pub fn new(tick_rate: u32, scale: u32) -> Self {
        Engine {
            id_factory: IdFactory::new(),
            status_map: HashMap::new(),
            collider_map: HashMap::new(),
            limitations_map: HashMap::new(),
            tick_rate,
            scale,
        }
    }
    pub fn update(&mut self) {
        self.status_map.iter_mut()
            .for_each(|(id, status)| {
                status.update(self.tick_rate);
                let limit = self.limitations_map.get(id).unwrap();
                if !limit.validate(status) {
                    limit.adjust_to_valid(status)
                }
            });
    }
    pub fn register(&mut self, object: Status, limitations: Limitations) -> u32 {
        let id = self.id_factory.next();
        self.status_map.insert(id, object);
        self.limitations_map.insert(id, limitations);
        return id;
    }

    pub fn register_collider(&mut self, id: u32, collider: CircleCollider2D) {
        self.collider_map.insert(id, collider);
    }

    pub fn check_collisions(&self) -> Vec<(u32, u32)> {
        let mut result = HashSet::new();
        for (id1, collider1) in &self.collider_map {
            let s_position = self.status_map.get(id1).unwrap().position;
            for (id2, collider2) in &self.collider_map {
                let c_position = self.status_map.get(id2).unwrap().position;
                if collider1.collide(collider2, &s_position, &c_position) && id1 != id2 {
                    if id1 < id2 {
                        result.insert((id1, id2));
                    } else {
                        result.insert((id2, id1));
                    }
                }
            }
        }
        result.iter()
            .map(|(a, b)| (**a, **b))
            .collect()
    }

    pub fn accelerate(&mut self, id: u32, acceleration: Vector2D) {
        let mut status = self.status_map.get_mut(&id).unwrap();
        status.accelerate(acceleration);
    }

    pub fn set_scale(&mut self, scale: u32) {
        self.scale = scale
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
            .map(|(id, status)| DirectedTriangle::equilateral(
                Point::new(
                    ((status.position.x as f32 / self.scale as f32) + ((width / 2) as f32)) as u32,
                    ((status.position.y as f32 / self.scale as f32) + ((height / 2) as f32)) as u32),
                Vector::new(status.speed.x as i32, status.speed.y as i32).normalize(),
                20))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .for_each(|triangle| triangle.paint(buffer));
    }
}