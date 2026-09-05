use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Component, Default)]
pub struct Emitter {
    stopwatch: Stopwatch,
}

impl Emitter {
    pub fn get_stopwatch(&mut self) -> &mut Stopwatch {
        &mut self.stopwatch
    }
}
