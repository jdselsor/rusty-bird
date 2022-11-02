use raylib::prelude::*;

use rusty_bird::{Application, RenderRect, Render};

fn main() {
    let app = Application::new(640, 960, "title");
    
    let mut handle = app.raylib_handle.borrow_mut();
    let rr = RenderRect { position: Vector2 { x: 640.0 / 2.0, y: 960.0 / 2.0 }, size: Vector2 { x:64.0 , y: 64.0 }};

    while !handle.window_should_close() {
        let mut d = handle.begin_drawing(&app.thread);

        rr.render(&mut d);

        d.clear_background(Color::WHITE);
    }
}