use std::cell::RefCell;

use raylib::prelude::*;

pub struct Application<'a> {
    width: i32,
    height: i32,
    _title: &'a str,
    pub raylib_handle: RefCell<RaylibHandle>,
    pub thread: RaylibThread,
}

impl Application<'_> {
    pub fn new (width: i32, height: i32, title: &str) -> Application {
        let (rl, thread) = raylib::init()
            .size(width, height)
            .title(title)
            .build();
        
        Application { width: width, height: height, _title: title, raylib_handle: RefCell::new(rl), thread: thread }
    }

    pub fn get_window_dimensions (&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn get_aspect_ratio (&self) -> f32 {
        (self.width as f32) / (self.height as f32)
    }
}

pub struct RenderRect {
    pub position: Vector2,
    pub size: Vector2,
}

impl Render for RenderRect {
    fn render (&self, context: &mut RaylibDrawHandle) {
        context.draw_rectangle_rec(Rectangle { x: self.position.x, y: self.position.y, width: self.size.x, height: self.size.y }, Color::GOLD);
    }
}

pub trait Render {
    fn render (&self, context: &mut RaylibDrawHandle);
}