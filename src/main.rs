use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::play::PlayPlugin;

mod constants;
mod emitter;
mod listener;
mod model;
mod movable;
mod play;

fn main() {
    App::new()
        .init_resource::<InputFocus>()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1600, 900).with_scale_factor_override(1.0),
                title: "Doppler".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FreeCameraPlugin)
        .add_plugins(PlayPlugin)
        .run();
}

pub fn setup(mut commands: Commands) {
    // camera at center looking toward neg-z
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        FreeCamera::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).looking_at(Vec3::NEG_Z, Vec3::Y),
    ));

    // light
    commands.spawn((
        DirectionalLight {
            illuminance: 20000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-1.0, -1.0, -1.0), Vec3::Y),
    ));
}
