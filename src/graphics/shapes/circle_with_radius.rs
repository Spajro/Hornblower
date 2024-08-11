use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::shapes::circle::Circle;
use crate::graphics::line::Line;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;

pub struct CircleWithRadius{
    pub circle:Circle,
    pub radius_direction:Normalized,
}

impl CircleWithRadius{
    pub fn new(center:Point,radius:u32,radius_direction:Normalized)->Self{
        CircleWithRadius{
            circle:Circle::new(center,radius),
            radius_direction
        }
    }
}

impl Paintable for CircleWithRadius{
    fn paint(&self, buffer: &mut Buffer) {
        let point=Point::from_vector(self.circle.center.as_vector()+self.radius_direction* self.circle.radius as i32);
        if point.is_ok()
        {
            self.circle.paint(buffer);
            Line::new(self.circle.center, point.unwrap()).paint(buffer);
        }
        else{
            println!("ERROR {}",point.err().unwrap_or("?"));
        }
    }
}