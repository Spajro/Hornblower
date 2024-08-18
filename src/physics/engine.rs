use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use crate::physics::collider::{CircleCollider2D, Collider};
use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::shapes::directed_triangle::DirectedTriangle;
use crate::graphics::point::Point;
use crate::graphics::shapes::circle_with_radius::CircleWithRadius;
use crate::graphics::vector::Vector;
use crate::physics::id::IdFactory;
use crate::physics::limitations::Limitations;
use crate::physics::normalized2d::Normalized2D;
use crate::physics::status::Status;
use crate::physics::vector2d::Vector2D;

pub struct Engine {
    id_factory: IdFactory,
    status_map: HashMap<u32, Status>,
    collider_map: HashMap<u32, CircleCollider2D>,
    limitations_map: HashMap<u32, Limitations>,
    object_type_map: HashMap<u32, ObjectType>,
    tick_rate: u32,
    scale: u32,
}

pub enum Event {
    Accelerate(u32, Normalized2D, f32),
    Scale(u32),
    Fire(u32, Normalized2D),
}

pub enum ObjectType {
    SHIP,
    MISSILE,
}

impl Engine {
    pub fn new(tick_rate: u32, scale: u32) -> Self {
        Engine {
            id_factory: IdFactory::new(),
            status_map: HashMap::new(),
            collider_map: HashMap::new(),
            limitations_map: HashMap::new(),
            object_type_map: HashMap::new(),
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
    pub fn register(&mut self, object: Status, limitations: Limitations, object_type: ObjectType) -> u32 {
        let id = self.id_factory.next();
        self.status_map.insert(id, object);
        self.limitations_map.insert(id, limitations);
        self.object_type_map.insert(id, object_type);
        return id;
    }

    pub fn register_collider(&mut self, id: u32, collider: CircleCollider2D) {
        self.collider_map.insert(id, collider);
    }

    pub fn check_collisions(&self) -> Vec<(u32, u32)> {
        let mut result = HashSet::new();
        for (id1, collider1) in &self.collider_map {
            let s_position = self.status_map.get(id1).unwrap().position();
            for (id2, collider2) in &self.collider_map {
                let c_position = self.status_map.get(id2).unwrap().position();
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

    pub fn handle_events(&mut self, events: Vec<Event>) {
        events.iter().for_each(|e|
            match e {
                Event::Accelerate(id, direction, percent) => {
                    let status = self.status_map.get_mut(id).unwrap();
                    let limit = self.limitations_map.get(id).unwrap();
                    status.accelerate(direction * (limit.acceleration() as f32 * percent) as i64)
                }
                Event::Scale(scale) => { self.scale = *scale }
                Event::Fire(id, direction) => {
                    let status = self.status_map.get_mut(id).unwrap();
                    let position = status.position() + direction * 10;
                    let speed = direction * 1000;
                    let missile = Status::with_position_and_speed(position, speed);
                    self.register(missile, Limitations::new(0, 1000), ObjectType::MISSILE);
                }
            }
        )
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
            .for_each(|(id, status)|
                {
                    let center = Point::new(
                        ((status.position().x as f32 / self.scale as f32) + ((width / 2) as f32)) as u32,
                        ((status.position().y as f32 / self.scale as f32) + ((height / 2) as f32)) as u32);
                    let direction = Vector::new(status.speed().x as i32, status.speed().y as i32).normalize();
                    match self.object_type_map.get(id).unwrap() {
                        ObjectType::SHIP => {
                            match DirectedTriangle::equilateral(center, direction, 20) {
                                Ok(t) => t.paint(buffer),
                                Err(_) => {}
                            }
                        }
                        ObjectType::MISSILE => {
                            CircleWithRadius::new(center, 5, direction).paint(buffer);
                        }
                    }
                }
            );
    }
}