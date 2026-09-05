use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::input_focus::InputFocus;
use bevy::light::Skybox;
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
        .add_systems(Update, close_on_esc)
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

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let skybox_handle = asset_server.load("textures/skybox.png");

    // camera at center looking toward neg-z
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        FreeCamera::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).looking_at(Vec3::NEG_Z, Vec3::Y),
        Skybox {
            image: Some(skybox_handle.clone()),
            brightness: 1000.0,
            ..default()
        },
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

fn close_on_esc(
    mut commands: Commands,
    focused_window: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (entity, window) in focused_window {
        if !window.focused {
            return;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(entity).despawn();
        }
    }
}
