use crate::physics::collider::CircleCollider2D;
use crate::physics::limitations::Limitations;

pub struct Cannon{
    pub missile_limit:Limitations,
    pub missile_collider:CircleCollider2D,
    pub reload_time:u32,
}

impl Cannon{
    pub fn new(missile_limit: Limitations, missile_collider: CircleCollider2D, reload_time: u32) -> Self {
        Self { missile_limit, missile_collider, reload_time }
    }

}