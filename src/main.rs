use bevy::prelude::*;
use orbiting_camera_plugin::{Orbit, OrbitingCameraPlugin};
mod orbiting_camera_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, keyboard_input)
        .add_systems(Update, light_orbiting)
        .add_plugins(OrbitingCameraPlugin)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    commands.spawn(SceneBundle{
        scene: asset_server.load("board.glb#Scene0"),
        ..Default::default()
    });
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut event_writer: EventWriter<Orbit>
) {
    // we can check multiple at once with `.any_*`
    if keys.any_pressed([KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D]) {
        let mut movement = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            movement = Vec3::new(0.0, time.delta_seconds(), 0.0);
        } else if keys.pressed(KeyCode::S) {
            movement = Vec3::new(0.0, -time.delta_seconds(), 0.0);
        } else if keys.pressed(KeyCode::A) {
            movement = Vec3::new(time.delta_seconds(), 0.0, 0.0);
        } else if keys.pressed(KeyCode::D) {
            movement = Vec3::new(-time.delta_seconds(), 0.0, 0.0);
        }
        
        event_writer.send(Orbit{ movement })
    }
}

fn light_orbiting(time: Res<Time>, mut query: Query<(&mut DirectionalLight , &mut Transform)>) {
        let (_, mut transform) = query.single_mut();
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(0.2 * time.delta_seconds()));
}