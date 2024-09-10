use crate::graphics::buffer::{Buffer, Color, Paintable};
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::shapes::circle_with_radius::CircleWithRadius;
use crate::graphics::shapes::directed_triangle::DirectedTriangle;
use crate::graphics::vector::Vector;
use crate::engine::engine::ObjectType;
use crate::engine::physics::Status;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }
    pub fn render(&self, objects: Vec<(ObjectType, &Status)>, scale: u32, buffer: &mut Buffer) {
        objects.iter()
            .for_each(|(object_type, status)|
                {
                    let center = Point::new(
                        ((status.position().x as f32 / scale as f32) + ((buffer.width / 2) as f32)) as u32,
                        ((status.position().y as f32 / scale as f32) + ((buffer.height / 2) as f32)) as u32);
                    let direction = Vector::new(status.speed().x as i32, status.speed().y as i32).normalize().unwrap_or(Normalized::new(0.0, -1.0));
                    match object_type {
                        ObjectType::SHIP => {
                            match DirectedTriangle::equilateral(center, direction, 20, Color::GREEN) {
                                Ok(t) => t.paint(buffer),
                                Err(_) => {}
                            }
                        }
                        ObjectType::MISSILE => {
                            CircleWithRadius::new(center, 5, direction, Color::BLUE).paint(buffer);
                        }
                    }
                }
            );
    }
}