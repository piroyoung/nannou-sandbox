use std::collections::VecDeque;

use nannou::prelude::App;
use nannou::rand::random_range;

pub struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity { x, y }
    }
}

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub v: Velocity,
}

impl Circle {
    pub fn step(&mut self) {
        self.x += self.v.x;
        self.y += self.v.y;
    }
}

pub struct Model {
    pub w: f32,
    pub h: f32,
    pub circles: VecDeque<Circle>,
}

impl Model {
    pub fn new(_app: &App) -> Model {
        Model {
            w: _app.window_rect().w(),
            h: _app.window_rect().h(),
            circles: VecDeque::new(),
        }
    }

    pub fn add_circle(&mut self, x: f32, y: f32, r: f32, v: Velocity) {
        self.circles.push_back(Circle { x, y, r, v });
    }

    pub fn trim_circles(&mut self, size: usize) {
        while self.circles.len() > size {
            self.circles.pop_front();
        }
    }

    pub fn step_circles(&mut self) {
        self.circles.iter_mut().for_each(|c| {
            c.step();
        })
    }

    pub fn add_circle_in_range(&mut self, r_min: f32, r_max: f32, mut v: Velocity) {
        let r = random_range(r_min, r_max);
        let shift_x = 0.5 * -self.w * v.x;
        let shift_y = 0.5 * -self.h * v.y;
        v.x *= 1.0 + r / (r_max - r_min);
        v.y *= 1.0 + r / (r_max - r_min);
        self.add_circle(
            random_range(-self.w / 2.0 + shift_x, self.w / 2.0 + shift_x),
            random_range(-self.h / 2.0 + shift_y, self.h / 2.0 + shift_y),
            r,
            v,
        )
    }
}