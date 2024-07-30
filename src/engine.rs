use std::fmt::{Display, Formatter};
use crate::status::Status;

pub struct Engine {
    pub objects: Vec<Status>,
    tick_rate: u32,
}

impl Engine {
    pub fn update(&mut self) {
        self.objects.iter_mut().for_each(|o| o.update(self.tick_rate))
    }
    pub fn register(&mut self, object: Status) {
        self.objects.push(object)
    }

    pub fn new(tick_rate: u32) -> Self {
        Engine {
            objects: vec![],
            tick_rate,
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