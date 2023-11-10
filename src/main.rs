use std::f32::consts::PI;

use bevy::prelude::*;
use orbiting_camera_plugin::{Orbit, OrbitingCameraPlugin};
use stl_loader_plugin::{StlLoaderPlugin};
mod orbiting_camera_plugin;
mod stl_loader_plugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StlLoaderPlugin))
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, keyboard_input)
        .add_systems(Update, light_orbiting)
        .add_plugins(OrbitingCameraPlugin)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(
    DirectionalLightBundle{
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 50000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 10.0,1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let card_bundle = PbrBundle {
        mesh: asset_server.load("card.stl"),
        material: materials.add(Color::rgb(0.9, 0.4, 0.3).into()),
        transform: Transform::from_translation(Vec3::new(-0.3, 0.0, 0.25))
            .with_rotation(Quat::from_rotation_x(- PI / 2.0)),
        ..Default::default()
    };

    let card_bundle_2 = PbrBundle {
        transform: Transform::from_translation(Vec3::new(-0.1, 0.0, 0.25))
            .with_rotation(Quat::from_rotation_x(- PI / 2.0)),
        ..card_bundle.clone()
    };

    commands.spawn(card_bundle);
    commands.spawn(card_bundle_2);

    commands.spawn(PbrBundle {
        mesh: asset_server.load("board.stl"),
        material: materials.add(Color::rgb(0.1, 0.4, 0.5).into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(- PI / 2.0)),
        ..Default::default()
    });
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut event_writer: EventWriter<Orbit>
) {
    // we can check multiple at once with `.any_*`
    if keys.pressed(KeyCode::W) {
        event_writer.send(Orbit{movement: Vec3::new(0.0, time.delta_seconds(), 0.0)});
    } else if keys.pressed(KeyCode::S) {
        event_writer.send(Orbit{movement: Vec3::new(0.0, -time.delta_seconds(), 0.0)});
    } else if keys.pressed(KeyCode::A) {
        event_writer.send(Orbit{movement: Vec3::new(time.delta_seconds(), 0.0, 0.0)});
    } else if keys.pressed(KeyCode::D) {
        event_writer.send(Orbit{movement: Vec3::new(-time.delta_seconds(), 0.0, 0.0)});
    }
}

fn light_orbiting(time: Res<Time>, mut query: Query<(&mut DirectionalLight , &mut Transform)>) {
        let (_, mut transform) = query.single_mut();
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(1.0 * time.delta_seconds()));
}