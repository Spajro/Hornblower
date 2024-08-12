use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use crate::physics::collider::CircleCollider2D;
use crate::physics::engine::Engine;
use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::gui::click::{Click, ClickHandler};
use crate::gui::compass::Compass;
use crate::gui::plus_minus::PlusMinus;
use crate::gui::throttle::Throttle;
use crate::gui::tick_button::TickButton;
use crate::physics::normalized2d::Normalized2D;
use crate::physics::status::Status;
use crate::physics::vector2d::Vector2D;


mod graphics;
mod physics;
mod gui;

fn normalized_to_normalized2d(normalized:Normalized)->Normalized2D{
    Normalized2D::new(normalized.x as f64, normalized.y as f64)
}

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

    let mut throttle=Throttle::new(Point::new(75,50),100,50);
    let mut button=TickButton::new(Point::new(125,25),25);
    let mut compass=Compass::new(
        Point::new(25,25),
        50,
    );
    let mut plusminus=PlusMinus::new(Point::new(200,25),50,Normalized::new(0.0,1.0),0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        //println!("|\n{}|", engine);
        if window.get_mouse_down(MouseButton::Left){
            let click=window.get_mouse_pos(MouseMode::Pass).unwrap();
            let click=Click::new(click.0 as u32, click.1 as u32);
            compass.handle_click(&click);
            throttle.handle_click(&click);
            button.handle_click(&click);
            plusminus.handle_click(&click);
            engine.accelerate(1,normalized_to_normalized2d(compass.direction)* ((40.0 * throttle.percent) as i64));
        }

        engine.update();
        let collisions = engine.check_collisions();
        if !collisions.is_empty() {
            let collision = collisions.first().unwrap();
            println!("COLLISION {} {}", collision.0, collision.1);
        };

        let mut buffer = Buffer::new(WIDTH, HEIGHT);
        compass.paint(&mut buffer);
        throttle.paint(&mut buffer);
        engine.paint(&mut buffer);
        button.paint(&mut buffer);
        plusminus.paint(&mut buffer);
        if button.clicked() {
            println!("CLICK");
        }

        window
            .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
            .unwrap();
    }
}
