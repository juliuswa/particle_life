use nannou::prelude::*;

#[derive(Clone)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Self {
            position: position,
            velocity: velocity,
        }
    }

    pub fn draw(&self, draw: &Draw) {
      
    }
}