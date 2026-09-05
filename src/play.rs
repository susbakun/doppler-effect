use bevy::prelude::*;

use crate::{
    model::{create_ambulance, create_road},
    movable::Movable,
};
pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, create_road)
            .add_systems(Startup, create_ambulance)
            .add_systems(Update, move_ambulance);
    }
}

fn move_ambulance(time: Res<Time>, mut query: Query<(&mut Transform, &Movable)>) {
    for (mut transform, object) in query.iter_mut() {
        let direction = Dir3::X;
        transform.translation += direction * object.get_speed() * time.delta_secs();
    }
}
