use bevy::prelude::*;

use crate::constants::AMBULANCE_SPEED;

#[derive(Component)]
pub struct Movable {
    speed: f32,
}

impl Movable {
    pub fn new() -> Self {
        Movable {
            speed: AMBULANCE_SPEED,
        }
    }

    pub const fn get_speed(&self) -> f32 {
        self.speed
    }
}
