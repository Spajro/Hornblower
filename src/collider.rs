use crate::vector2d::Vector2D;

pub trait Collider<T> {
    fn collide(&self, t: &T, s_position: &Vector2D, t_position: &Vector2D) -> bool;
}

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
        (distance <= self.radius - collider.radius) ||
            (distance <= collider.radius - self.radius) ||
            (distance <= self.radius + collider.radius)
    }
}