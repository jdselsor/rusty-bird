use std::borrow::{Borrow, BorrowMut};

use raylib::prelude::*;

use rusty_bird::{Application, Pipe, Bird, Entity};

fn main() {
    let app = Application::new(640, 960, "Rusty Bird");
    
    let mut handle = app.raylib_handle.borrow_mut();

    let mut entites: Vec<Box<dyn Entity>> = Vec::new();

    let mut player_bird = Bird::new(Vector2 {x: 640.0 / 2.0, y: 960.0 / 2.0}, 24.0, Color::DARKGREEN);
    let mut temp_pipe = Pipe::new (Vector2 { x: 20.0, y: 64.0 }, Vector2 { x: 10.0, y: 34.0 }, Color::GREEN);

    entites.push(Box::new(player_bird));
    entites.push(Box::new(temp_pipe));

    while !handle.window_should_close() {
        let mut draw_handle = handle.begin_drawing(&app.thread);

        for ent in entites.iter_mut() {
            ent.update(&mut handle);
            ent.render(&mut draw_handle);
        }

        draw_handle.clear_background(Color::WHITE);
    }
}