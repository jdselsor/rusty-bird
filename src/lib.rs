use std::cell::{RefCell, RefMut, Ref};

use raylib::prelude::*;

const GRAVITY: f32 = 1.0;

pub struct Application<'a> {
    width: i32,
    height: i32,
    _title: &'a str,
    pub raylib_handle: RefCell<RaylibHandle>,
    pub thread: RaylibThread,
}

pub struct Bird {
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f32,
    pub color: Color,
}

pub struct Pipe {
    pub position: Vector2,
    pub velocity: Vector2,
    pub dimensions: Vector2,
    pub color: Color,
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

impl Bird {
    pub fn new (position: Vector2, radius: f32, color: Color) -> Bird {
        Bird { position: position, velocity: Vector2 {x: 0.0, y: 0.0}, radius: radius, color: color }
    }

    pub fn set_velocity (&mut self, x: f32, y: f32) {
        self.velocity.x = x;
        self.velocity.y = y;
    }
}

impl Pipe {
    pub fn new (position: Vector2, dimensions: Vector2, color: Color) -> Pipe {
        Pipe { position: position, velocity: Vector2 { x: 0.0, y: 0.0 }, dimensions: dimensions, color: color }
    } 
}

impl Entity for Pipe {
    fn render (&self, context: &mut RaylibDrawHandle) {
        context.draw_rectangle_v(self.position, self.dimensions, self.color);
    }

    fn update (&mut self) {
        
    }
}

impl Entity for Bird {
    fn render (&self, context: &mut RaylibDrawHandle) {
        context.draw_circle_v(self.position, self.radius, self.color);
    }

    fn update (&mut self) {
        self.velocity.y += GRAVITY;
        self.position = self.position + self.velocity;
    }
}

pub trait Entity {
    fn render (&self, context: &mut RaylibDrawHandle);
    fn update (&mut self);
}