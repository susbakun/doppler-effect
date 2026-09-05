use bevy::prelude::*;

use crate::movable::Movable;

pub fn create_road(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/road.glb"))),
        Transform {
            translation: Vec3::new(0.0, -2.5, -4.0),
            scale: Vec3::splat(2.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI / 2.0),
        },
    ));
}

pub fn create_ambulance(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/ambulance.glb")),
        ),
        Transform {
            translation: Vec3::new(-5.0, -0.5, -4.0),
            scale: Vec3::splat(0.02),
            rotation: Quat::from_rotation_y(std::f32::consts::PI / 2.0),
        },
        Movable::new(Vec3::new(-5.0, -0.5, -4.0)),
    ));
}
