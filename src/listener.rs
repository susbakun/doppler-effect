use bevy::prelude::*;

pub fn setup_listener(mut commands: Commands) {
    // Space between the two ears
    let gap = 4.0;
    let listener = SpatialListener::new(gap);

    commands.spawn((
        Transform::default(),
        Visibility::default(),
        listener.clone(),
        children![
            // left ear indicator
            Transform::from_translation(listener.left_ear_offset),
            // right ear indicator
            Transform::from_translation(listener.right_ear_offset)
        ],
    ));
}
