use crate::play::PlayPlugin;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::input_focus::InputFocus;
use bevy::light::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use bevy::window::WindowResolution;

mod constants;
mod doppler;
mod emitter;
mod listener;
mod model;
mod movable;
mod play;

fn main() {
    App::new()
        .init_resource::<InputFocus>()
        .init_resource::<Cubemap>()
        .add_systems(Startup, setup)
        .add_systems(Startup, load_skybox_to_cubemap)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, asset_loaded)
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

#[derive(Resource, Default)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

fn load_skybox_to_cubemap(mut cubemap: ResMut<Cubemap>, asset_server: Res<AssetServer>) {
    cubemap.image_handle = asset_server.load("textures/skybox.png");
    cubemap.is_loaded = false;
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skybox: Single<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
        let mut image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            let layers = image.height() / image.width();
            image
                .reinterpret_stacked_2d_as_array(layers)
                .expect("asset should be 2d texture and height will always be evenly divisible with the given layers");
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        skybox.image = Some(cubemap.image_handle.clone());
        cubemap.is_loaded = true;
    }
}
