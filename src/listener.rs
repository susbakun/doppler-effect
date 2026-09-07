use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

pub fn setup_listener(mut commands: Commands) {
    // Space between the two ears
    let gap = 4.0;
    let listener = SpatialListener::new(gap);

    commands.spawn((
        Transform::default(),
        Visibility::default(),
        listener.clone(),
        Velocity::default(),
        children![
            // left ear indicator
            Transform::from_translation(listener.left_ear_offset),
            // right ear indicator
            Transform::from_translation(listener.right_ear_offset)
        ],
    ));
}

pub fn update_listener(
    time: Res<Time>,
    mut listener: Query<(&mut Transform, &mut Velocity), With<SpatialListener>>,
    camera: Single<&Transform, (With<Camera3d>, Without<SpatialListener>)>,
) {
    let Ok((mut transform, mut velocity)) = listener.single_mut() else {
        return;
    };

    update_listener_velocity(
        time,
        // new pos
        camera.translation,
        // old pos
        transform.translation,
        &mut velocity,
    );

    transform.translation = camera.translation;
    transform.rotation = camera.rotation;
}

fn update_listener_velocity(
    time: Res<Time>,
    new_pos: Vec3,
    old_pos: Vec3,
    velocity: &mut Velocity,
) {
    let delta = time.delta_secs();

    if delta <= 0.0 {
        return;
    }

    velocity.0 = (new_pos - old_pos) / delta
}
