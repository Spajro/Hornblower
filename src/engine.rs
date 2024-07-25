use std::fmt::{Display, Formatter};
use crate::status::Status;

pub struct Engine {
    pub objects: Vec<Status>,
}

impl Engine {
    pub fn update(&mut self) {
        self.objects.iter_mut().for_each(|o| o.update())
    }
    pub fn register(&mut self, object: Status) {
        self.objects.push(object)
    }

    pub fn new() -> Self {
        Engine {
            objects: vec![],
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