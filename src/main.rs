use std::thread::sleep;
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use crate::engine::Engine;
use crate::graphics::buffer::{Buffer, Paintable};
use crate::status::Status;
use crate::vector2d::Vector2D;

mod status;
mod engine;
mod vector2d;
mod normalized2d;

mod graphics;


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

    let mut status1 = Status::new();
    status1.accelerate(Vector2D::new(10, 0));
    status1.position.x = 100;
    status1.position.y = 100;
    engine.register(status1);
    let mut status2 = Status::new();
    status2.accelerate(Vector2D::new(0, 10));
    status2.position.x = 200;
    status2.position.y = 200;
    engine.register(status2);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        println!("|\n{}|", engine);
        sleep(Duration::from_millis(FRAME_TIME as u64));

        let mut buffer = Buffer::new(WIDTH, HEIGHT);
        engine.update();
        engine.paint(&mut buffer);

        window
            .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
            .unwrap();
    }
}
