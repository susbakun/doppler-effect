use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;

use crate::{
    constants::MOUSE_SENSITIVITY,
    emitter::Emitter,
    listener::{setup_listener, update_listener},
    model::{create_ambulance, create_road},
    movable::Movable,
};
pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup_listener)
            .add_systems(Startup, create_road)
            .add_systems(Startup, create_ambulance)
            .add_systems(Update, mouse_look)
            .add_systems(Update, update_listener)
            .add_systems(Update, move_ambulance);
    }
}

fn move_ambulance(time: Res<Time>, mut query: Query<(&mut Transform, &mut Emitter, &Movable)>) {
    for (mut transform, mut emitter, object) in query.iter_mut() {
        let direction = Dir3::X;
        transform.translation += direction * object.get_speed() * time.delta_secs();

        emitter.get_stopwatch().tick(time.delta());
    }
}

// controlling camera using mouse
fn mouse_look(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    let delta = mouse_motion.delta;

    if delta == Vec2::ZERO {
        return;
    }

    let yaw = -delta.x * MOUSE_SENSITIVITY;
    let pitch = -delta.y * MOUSE_SENSITIVITY;

    camera.rotate_y(yaw);
    camera.rotate_local_x(pitch);
}
