use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::square::Square;
use crate::gui::click::{Click, ClickHandler};

pub struct Button {
    pub center: Point,
    pub size: u32,
    pub direction: Normalized,
    click: bool,
}

impl Button {
    pub fn new(center: Point, size: u32) -> Self {
        Button {
            center,
            size,
            direction: Normalized::new(0.0, -1.0),
            click: false,
        }
    }

    pub fn clicked(&self)->bool{
        self.click
    }

    pub fn reset(&mut self){
        self.click=false;
    }
}

impl Paintable for Button {
    fn paint(&self, buffer: &mut Buffer) {
        Square::new(self.center, Normalized::new(0.0, -1.0), 2 * self.size).paint(buffer);
        Square::new(self.center, Normalized::new(0.0, -1.0), (1.8 * self.size as f32) as u32).paint(buffer);
    }
}

impl ClickHandler for Button {
    fn handle_click(&mut self, click: &Click) {
        if click.x < self.center.x as u32 - (self.size / 2) ||
            click.x > self.center.x as u32 + (self.size / 2) ||
            click.y < self.center.y as u32 - (self.size / 2) ||
            click.y > self.center.y as u32 + (self.size / 2)
        {
            return;
        }
        self.click = true;
    }
}