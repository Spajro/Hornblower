use std::time::{Duration, Instant};

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use crate::game::interface::Interface;
use crate::game::render::Renderer;
use crate::graphics::buffer::{Buffer, Paintable};
use crate::gui::click::Click;
use crate::physics::cannon::Cannon;
use crate::physics::collider::CircleCollider2D;
use crate::physics::engine::{Engine, ObjectType};
use crate::physics::limitations::Limitations;
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
    let mut renderer=Renderer::new();
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| { panic!("{}", e); });
    window.set_target_fps(FRAME_RATE as usize);

    let status = Status::with_position(Vector2D::new(100, 100));
    let limit = Limitations::new(100, 1000);
    let id = engine.register_with_collider(status, limit, ObjectType::SHIP, CircleCollider2D::new(20));
    engine.register_cannon(id, Cannon::new(Limitations::new(0, 500), CircleCollider2D::new(50), FRAME_RATE * 3));

    let mut interface = Interface::new(HEIGHT as u32, WIDTH as u32, SCALE);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        //println!("|\n{}|", engine);
        let now = Instant::now();
        if window.get_mouse_down(MouseButton::Left) {
            let click = window.get_mouse_pos(MouseMode::Pass).unwrap();
            let click = &Click::new(click.0 as u32, click.1 as u32);
            engine.handle_events(interface.handle_click(click, id));
        }

        let now_update = Instant::now();
        engine.update();
        let elapsed_update = now_update.elapsed();
        let collisions = engine.check_collisions();
        if !collisions.is_empty() {
            let collision = collisions.first().unwrap();
            println!("COLLISION {} {}", collision.0, collision.1);
        };


        let now_paint = Instant::now();
        let mut buffer = Buffer::new(WIDTH, HEIGHT);
        renderer.render(engine.get_renderable(),engine.scale(),&mut buffer);
        interface.paint(&mut buffer);
        let elapsed_paint = now_paint.elapsed();

        let elapsed_time = now.elapsed();
        println!("Tpf:{}, Tpu:{}, Tpp:{}, fps:{}",
                 elapsed_time.as_micros(),
                 elapsed_update.as_micros(),
                 elapsed_paint.as_micros(),
                 Duration::from_secs(1).as_nanos() / elapsed_time.as_nanos());

        window
            .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
            .unwrap();
    }
}
