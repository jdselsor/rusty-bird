extern crate raylib;

use raylib::prelude::*;

use rusty_bird::{Application, Render, Bird, Update};

fn main() {
    let app = Application::new(640, 960, "Rusty Bird");
    
    let mut handle = app.raylib_handle.borrow_mut();
    //let mut player_bird = Bird{ position: Vector2 { x: 640.0/2.0, y: 960.0/2.0 }, radius: 24.0, color: Color::DARKPURPLE };
    let mut player_bird = Bird::new(Vector2 {x: 640.0 / 2.0, y: 960.0 / 2.0}, 24.0, Color::DARKGREEN);

    while !handle.window_should_close() {
        if handle.is_key_pressed(KeyboardKey::KEY_SPACE) {
            player_bird.set_velocity(0.0, -25.0);
        }

        player_bird.update();

        let mut d = handle.begin_drawing(&app.thread);

        player_bird.render(&mut d);

        d.clear_background(Color::WHITE);
    }
}