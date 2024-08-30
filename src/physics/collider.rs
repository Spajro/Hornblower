use crate::physics::vector2d::Vector2D;

pub trait Collider<T> {
    fn collide(&self, t: &T, s_position: &Vector2D, t_position: &Vector2D) -> bool;
}

#[derive(Clone)]
pub struct CircleCollider2D {
    radius: u32,
}

impl CircleCollider2D {
    pub fn new(radius: u32) -> Self {
        CircleCollider2D {
            radius
        }
    }
}

impl Collider<CircleCollider2D> for CircleCollider2D {
    fn collide(&self, collider: &CircleCollider2D, s_position: &Vector2D, c_position: &Vector2D) -> bool {
        let distance = s_position.distance(c_position) as u32;
        (distance as i32 <= self.radius as i32 - collider.radius as i32) ||
            (distance as i32 <= collider.radius as i32 - self.radius as i32) ||
            (distance <= self.radius + collider.radius)
    }
}