use bevy::prelude::*;

use crate::constants::AMBULANCE_SPEED;

#[derive(Component)]
pub struct Movable {
    speed: f32,
    velocity: Vec3,
}

impl Movable {
    pub fn new() -> Self {
        Movable {
            speed: AMBULANCE_SPEED,
            velocity: Vec3::ZERO,
        }
    }

    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    pub const fn get_velocity(&self) -> Vec3 {
        self.velocity
    }

    pub const fn get_speed(&self) -> f32 {
        self.speed
    }
}
