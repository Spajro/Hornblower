use crate::engine::physics::Status;
use crate::engine::math::Vector2D;


#[derive(Clone)]
pub struct Limitations {
    acceleration: u64,
    speed: u64,
}

impl Limitations {
    pub fn new(acceleration: u64, speed: u64) -> Self {
        Self { acceleration, speed }
    }

    pub fn validate(&self, status: &Status) -> bool {
        self.acceleration >= status.acceleration().length() && self.speed >= status.speed().length()
    }

    pub fn adjust_to_valid(&self, status: &mut Status) {
        if self.acceleration < status.acceleration().length() {
            Self::adjust(self.acceleration, &mut status.acceleration())
        }
        if self.speed < status.speed().length() {
            Self::adjust(self.speed, &mut status.speed())
        }
    }

    fn adjust(limit: u64, vector: &mut Vector2D) {
        let len = vector.length();
        vector.x = limit as i64 * vector.x / len as i64;
        vector.y = limit as i64 * vector.y / len as i64;
    }

    pub fn acceleration(&self)->u64{
        self.acceleration
    }

    pub fn speed(&self)->u64{
        self.speed
    }
}