use std::thread::sleep;
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use crate::physics::collider::CircleCollider2D;
use crate::physics::engine::Engine;
use crate::graphics::buffer::{Buffer, Paintable};
use crate::physics::status::Status;
use crate::physics::vector2d::Vector2D;



mod graphics;
mod physics;


fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;
    const TICK_RATE:u32=4;
    const FRAME_TIME:u32=1000/TICK_RATE;
    const SCALE:u32=4;

    let mut engine = Engine::new(TICK_RATE,SCALE);
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| { panic!("{}", e); });
    window.set_target_fps(60);

    let mut status1 = Status::with_position(Vector2D::new(100,100));
    status1.accelerate(Vector2D::new(-10, 0));
    engine.register(1,status1);
    engine.register_collider(1,CircleCollider2D::new(20));

    let mut status2 = Status::with_position(Vector2D::new(200,200));
    status2.accelerate(Vector2D::new(0, -10));
    engine.register(2,status2);
    engine.register_collider(2,CircleCollider2D::new(20));


    while window.is_open() && !window.is_key_down(Key::Escape) {
        println!("|\n{}|", engine);
        sleep(Duration::from_millis(FRAME_TIME as u64));

        let mut buffer = Buffer::new(WIDTH, HEIGHT);
        engine.update();
        let collisions=engine.check_collisions();
        if !collisions.is_empty(){
            let collision=collisions.first().unwrap();
            println!("COLLISION {} {}",collision.0,collision.1);
        };
        engine.paint(&mut buffer);

        window
            .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
            .unwrap();
    }
}
