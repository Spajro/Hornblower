use std::thread::sleep;
use std::time::Duration;
use crate::engine::Engine;
use crate::status::Status;
use crate::vector2d::Vector2D;


mod status;
mod engine;
mod vector2d;
mod normalized2d;


fn main() {
    let mut engine=Engine::new();
    let mut status =Status::new();
    status.accelerate(Vector2D::new(100,0));
    engine.register(status);
    for _ in 0..100{
        engine.update();
        sleep(Duration::from_secs(1));
        println!("|{}|", engine)
    }
}
