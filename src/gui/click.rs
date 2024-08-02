use crate::graphics::vector::Vector;

pub struct Click {
    pub x: u32,
    pub y: u32,
}

pub trait ClickHandler {
    fn handle_click(&mut self, click: &Click);
}

impl Click {
    pub fn new(x: u32, y: u32) -> Self {
        Click {
            x,
            y,
        }
    }

    pub fn as_vector(&self) -> Vector {
        Vector::new(self.x as i32, self.y as i32)
    }
}