use bevy::prelude::*;

use crate::constants::AMBULANCE_SPEED;

#[derive(Component)]
pub struct Movable {
    start: Vec3,
    speed: f32,
}

impl Movable {
    pub fn new(start: Vec3) -> Self {
        Movable {
            start,
            speed: AMBULANCE_SPEED,
        }
    }

    pub const fn get_speed(&self) -> f32 {
        self.speed
    }
}
