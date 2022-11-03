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
        let (mut rl, thread) = raylib::init()
            .size(width, height)
            .title(title)
            .build();
        
        rl.set_target_fps(60);

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

pub struct Bird {
    pub position: Vector2,
    pub radius: f32,
    pub color: Color,
}

impl Render for Bird {
    fn render (&self, context: &mut RaylibDrawHandle) {
        context.draw_circle_v(self.position, self.radius, self.color);
    }
}

impl Update for Bird {
    fn update (&self) {
        println!("Updating Bird");
    }
}

impl Render for RenderRect {
    fn render (&self, context: &mut RaylibDrawHandle) {
        context.draw_rectangle_rec(Rectangle { x: self.position.x, y: self.position.y, width: self.size.x, height: self.size.y }, Color::GOLD);
    }
}

pub trait Render {
    fn render (&self, context: &mut RaylibDrawHandle);
}

pub trait Update {
    fn update (&self);
}