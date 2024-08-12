use crate::graphics::buffer::{Buffer, Paintable};

pub struct Figure {
    parts: Vec<Box<dyn Paintable>>,
}

impl Figure {
    pub fn new() -> Self {
        Figure {
            parts: vec![],
        }
    }

    pub fn with(mut self, p: Box<dyn Paintable>) -> Self {
        self.parts.push(p);
        self
    }

    pub fn join(mut self, mut figure: Figure) ->Self{
        self.parts.append(&mut figure.parts);
        self
    }
}

impl Paintable for Figure {
    fn paint(&self, buffer: &mut Buffer) {
        self.parts.iter().for_each(|p| p.paint(buffer));
    }
}