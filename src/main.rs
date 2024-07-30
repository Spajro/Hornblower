use std::thread::sleep;
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use crate::engine::Engine;
use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::point::Point;
use crate::graphics::triangle::Triangle;
use crate::graphics::vector::Vector;
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
    let mut engine = Engine::new();
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| { panic!("{}", e); });

    window.set_target_fps(60);

    let mut status1 = Status::new();
    status1.accelerate(Vector2D::new(5, 0));
    status1.position.x = 100;
    status1.position.y = 100;
    engine.register(status1);
    let mut status2 = Status::new();
    status2.accelerate(Vector2D::new(0, 5));
    status2.position.x = 200;
    status2.position.y = 200;
    engine.register(status2);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        println!("|{}|", engine);
        sleep(Duration::from_secs(1));

        let mut buffer = Buffer::new(WIDTH, HEIGHT);
        engine.update();


        engine.objects.iter()
            .map(|status| Triangle::equilateral(
                Point::new(status.position.x.clone() as usize, status.position.y.clone() as usize),
                Vector::new(status.speed.x.clone() as i32, status.speed.y.clone() as i32).normalize(),
                50))
            .for_each(|triangle|triangle.paint(&mut buffer));

        window
            .update_with_buffer(&buffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
