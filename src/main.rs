use raylib::prelude::*;

use rusty_bird::{Application, RenderRect, Render, Bird};

fn main() {
    let app = Application::new(640, 960, "title");
    
    let mut handle = app.raylib_handle.borrow_mut();
    let mut player_bird = Bird{ position: Vector2 { x: 640.0/2.0, y: 960.0/2.0 }, radius: 24.0, color: Color::DARKPURPLE };

    while !handle.window_should_close() {
        player_bird.position.y -= 0.5;

        let mut d = handle.begin_drawing(&app.thread);

        player_bird.render(&mut d);

        d.clear_background(Color::WHITE);
    }
}