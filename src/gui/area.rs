use crate::gui::click::Click;

pub struct Area {
    x1: u32,
    x2: u32,
    y1: u32,
    y2: u32,
}

impl Area {
    pub fn new(x1:u32,y1:u32,x2:u32,y2:u32)->Self{
        Area{
            x1,
            x2,
            y1,
            y2,
        }
    }
    pub fn is_in(&self, click: &Click) -> bool {
        click.x > self.x1 && click.x < self.x2 && click.y > self.y1 && click.y < self.y2
    }
}