use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

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

    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(0.0, 0.4,0.4).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(SceneBundle{
        scene: asset_server.load("board.glb#Scene0"),
        ..Default::default()
    });
}