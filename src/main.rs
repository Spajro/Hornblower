use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use crate::physics::collider::CircleCollider2D;
use crate::physics::engine::Engine;
use crate::graphics::buffer::{Buffer, Paintable};
use crate::graphics::circle_with_radius::CircleWithRadius;
use crate::graphics::normalized::Normalized;
use crate::graphics::point::Point;
use crate::graphics::rectangle::Rectangle;
use crate::graphics::vector::Vector;
use crate::gui::click::Click;
use crate::gui::compass::Compass;
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
    const SCALE: u32 = 4;

    let mut engine = Engine::new(FRAME_RATE, SCALE);
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| { panic!("{}", e); });
    window.set_target_fps(FRAME_RATE as usize);

    let mut status1 = Status::with_position(Vector2D::new(100, 100));
    status1.accelerate(Vector2D::new(-10, 0));
    engine.register(1, status1);
    engine.register_collider(1, CircleCollider2D::new(20));

    let mut status2 = Status::with_position(Vector2D::new(200, 200));
    status2.accelerate(Vector2D::new(0, -10));
    engine.register(2, status2);
    engine.register_collider(2, CircleCollider2D::new(20));

    let circle=CircleWithRadius::new(Point::new(200,200),100,Vector::new(45,45).normalize());
    let rectangle=Rectangle::new(Point::new(250,250),Normalized::new(0.5,0.5),100,50);
    let mut compass=Compass::new(
        Point::new(25,25),
        50,
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        println!("|\n{}|", engine);
        if window.get_mouse_down(MouseButton::Left){
            let click=window.get_mouse_pos(MouseMode::Pass).unwrap();
            println!("CLICK {} {}",click.0,click.1);
            let click=Click::new(click.0 as u32, click.1 as u32);
            compass.handle_click(click);
            engine.accelerate(1,normalized_to_normalized2d(compass.direction)*40);
        }

        let mut buffer = Buffer::new(WIDTH, HEIGHT);
        circle.paint(&mut buffer);
        compass.paint(&mut buffer);
        rectangle.paint(&mut buffer);
        engine.update();
        let collisions = engine.check_collisions();
        if !collisions.is_empty() {
            let collision = collisions.first().unwrap();
            println!("COLLISION {} {}", collision.0, collision.1);
        };
        engine.paint(&mut buffer);

        window
            .update_with_buffer(&buffer.buffer, buffer.width, buffer.height)
            .unwrap();
    }
}
