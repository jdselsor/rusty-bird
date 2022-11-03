use std::cell::RefCell;

use raylib::prelude::*;

const GRAVITY: f32 = 1.0;

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

pub struct Bird {
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f32,
    pub color: Color,
}

impl Bird {
    fn new (position: Vector2, radius: f32, color: Color) -> Bird {
        Bird { position: position, velocity: Vector2 {x: 0.0, y: 0.0}, radius: radius, color: color }
    }
}

impl Render for Bird {
    fn render (&self, context: &mut RaylibDrawHandle) {
        context.draw_circle_v(self.position, self.radius, self.color);
    }
}

impl Update for Bird {
    fn update (&mut self) {
        self.position.y += GRAVITY;
    }
}

pub trait Render {
    fn render (&self, context: &mut RaylibDrawHandle);
}

pub trait Update {
    fn update (&mut self);
}