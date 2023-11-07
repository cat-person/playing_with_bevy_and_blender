use bevy::prelude::*;

#[derive(Event)]
pub struct Move(Entity);

#[derive(Event)]
pub struct Zoom(Entity);

#[derive(Event)]
pub struct Orbit{
    pub movement: Vec3
}

pub struct OrbitingCameraPlugin;

impl Plugin for OrbitingCameraPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, setup_camera)
            .add_systems(Update, listen_for_events)
            .add_event::<Move>()
            .add_event::<Zoom>()
            .add_event::<Orbit>();
            
	}
}

fn setup_camera(mut commands: Commands){
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.5, 0.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn listen_for_events(mut event_reader: EventReader<Orbit>, mut query: Query<(&mut Camera3d, &mut Transform)>) {
    for event in event_reader.read() {
        let (_, mut transform) = query.single_mut();
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(event.movement.x));
        transform.translation = (transform.translation - Vec3::ZERO) * (1.0 - event.movement.y); 

        println!("{}", event.movement)
    }
}