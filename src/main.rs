use std::time::{Duration, Instant};

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use crate::game::interface::Interface;
use crate::graphics::buffer::{Buffer, Paintable};
use crate::gui::click::{Click, ClickHandler};
use crate::physics::collider::CircleCollider2D;
use crate::physics::engine::Engine;
use crate::physics::status::Status;
use crate::physics::vector2d::Vector2D;

mod graphics;
mod physics;
mod gui;
mod game;


fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;
    const FRAME_RATE: u32 = 10;
    const SCALE: u32 = 10;

    let mut engine = Engine::new(FRAME_RATE, SCALE);
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| { panic!("{}", e); });
    window.set_target_fps(FRAME_RATE as usize);

    let status1 = Status::with_position(Vector2D::new(100, 100));
    engine.register(1, status1);
    engine.register_collider(1, CircleCollider2D::new(20));

    let mut interface = Interface::new(HEIGHT as u32, WIDTH as u32, SCALE);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        //println!("|\n{}|", engine);
        let now = Instant::now();
        if window.get_mouse_down(MouseButton::Left) {
            let click = window.get_mouse_pos(MouseMode::Pass).unwrap();
            let click = &Click::new(click.0 as u32, click.1 as u32);
            interface.handle_click(click, &mut engine,1);
        }

        engine.update();
        let collisions = engine.check_collisions();
        if !collisions.is_empty() {
            let collision = collisions.first().unwrap();
            println!("COLLISION {} {}", collision.0, collision.1);
        };

        let mut buffer = Buffer::new(WIDTH, HEIGHT);
        engine.paint(&mut buffer);
        interface.paint(&mut buffer);

        let elapsed_time = now.elapsed();
        println!("Time per frame:{} , fps:{}", elapsed_time.as_nanos(), Duration::from_secs(1).as_nanos() / elapsed_time.as_nanos());

        window
            .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
            .unwrap();
    }
}
