use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use crate::physics::cannon::Cannon;
use crate::physics::collider::{CircleCollider2D, Collider};
use crate::physics::float_vector2d::FloatVector2D;
use crate::physics::id::{ID, IdFactory};
use crate::physics::limitations::Limitations;
use crate::physics::normalized2d::Normalized2D;
use crate::physics::status::Status;

pub struct Engine {
    id_factory: IdFactory,
    status_map: HashMap<ID, Status>,
    carry_map: HashMap<ID, FloatVector2D>,
    collider_map: HashMap<ID, CircleCollider2D>,
    limitations_map: HashMap<ID, Limitations>,
    object_type_map: HashMap<ID, ObjectType>,
    cannon_map: HashMap<ID, Cannon>,
    last_fired_map: HashMap<ID, u32>,
    tick_rate: u32,
    last_tick: u32,
    scale: u32,
}

pub enum Event {
    Accelerate(ID, Normalized2D, f32),
    Scale(u32),
    Fire(ID, Normalized2D),
}

#[derive(Copy,Clone)]
pub enum ObjectType {
    SHIP,
    MISSILE,
}

impl Engine {
    pub fn new(tick_rate: u32, scale: u32) -> Self {
        Engine {
            id_factory: IdFactory::new(),
            status_map: HashMap::new(),
            carry_map: HashMap::new(),
            collider_map: HashMap::new(),
            limitations_map: HashMap::new(),
            object_type_map: HashMap::new(),
            cannon_map: HashMap::new(),
            last_fired_map: HashMap::new(),
            tick_rate,
            last_tick: 0,
            scale,
        }
    }
    pub fn update(&mut self) {
        self.status_map.iter_mut()
            .for_each(|(id, status)| {
                let old_carry = self.carry_map.get(id).unwrap();
                let new_carry = status.update(old_carry, self.tick_rate);
                self.carry_map.insert(*id, new_carry);
                let limit = self.limitations_map.get(id).unwrap();
                if !limit.validate(status) {
                    limit.adjust_to_valid(status)
                }
            });
        self.last_tick += 1;
    }
    pub fn register(&mut self, object: Status, limitations: Limitations, object_type: ObjectType) -> ID {
        let id = self.id_factory.next();
        self.status_map.insert(id, object);
        self.carry_map.insert(id, FloatVector2D::new(0.0, 0.0));
        self.limitations_map.insert(id, limitations);
        self.object_type_map.insert(id, object_type);
        return id;
    }

    pub fn register_with_collider(&mut self, object: Status, limitations: Limitations, object_type: ObjectType, collider: CircleCollider2D) -> ID {
        let id = self.register(object, limitations, object_type);
        self.collider_map.insert(id, collider);
        id
    }

    pub fn register_cannon(&mut self, id: ID, cannon: Cannon) {
        self.cannon_map.insert(id, cannon);
        self.last_fired_map.insert(id, 0);
    }

    pub fn check_collisions(&self) -> Vec<(ID, ID)> {
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
                    let cannon = self.cannon_map.get(id).unwrap();
                    let last_fired = self.last_fired_map.get(id).unwrap();
                    if last_fired + cannon.reload_time > self.last_tick {
                        return;
                    }
                    self.last_fired_map.insert(*id, self.last_tick);

                    let position = status.position() + direction * cannon.missile_collider.radius as i64;
                    let speed = direction * cannon.missile_limit.speed() as i64;
                    let missile = Status::with_position_and_speed(position, speed);
                    self.register_with_collider(missile, cannon.missile_limit.clone(), ObjectType::MISSILE, cannon.missile_collider.clone());
                }
            }
        )
    }

    pub fn get_renderable(&self) -> Vec<(ObjectType, &Status)> {
        self.status_map.iter()
            .map(|(id, status)| (*self.object_type_map.get(id).unwrap(), status))
            .collect()
    }


    pub fn scale(&self) -> u32 {
        self.scale
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(
            for (id, status) in &self.status_map {
                write!(f, "{} : {}\n", id, status).unwrap();
            })
    }
}
