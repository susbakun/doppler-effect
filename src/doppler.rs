use bevy::prelude::*;

use crate::{constants::SOUND_OF_SPEED, listener::Velocity, movable::Movable};

pub fn update_doppler(
    mut ambulance: Single<(&Transform, &Movable, &mut SpatialAudioSink)>,
    listener: Single<(&Transform, &Velocity), With<SpatialListener>>,
) {
    let (listner_transform, listener_velocity) = (listener.0, listener.1);
    let (amb_transform, amb_speed, amb_audio) = (ambulance.0, ambulance.1, &mut ambulance.2);

    let listener_pos = listner_transform.translation;
    let amb_position = amb_transform.translation;

    let source_to_listner = (listener_pos - amb_position).normalize();
    let source_velocity = amb_speed.get_velocity();
    // Positive when ambulance moves toward listener.
    let source_towards_listener = source_velocity.dot(source_to_listner);

    let listner_to_source = (amb_position - listener_pos).normalize();
    // Positive when listener moves toward ambulance.
    let listener_towards_source = listener_velocity.0.dot(listner_to_source);

    let ratio =
        (SOUND_OF_SPEED + listener_towards_source) / (SOUND_OF_SPEED - source_towards_listener);

    amb_audio.set_speed(ratio);
}
