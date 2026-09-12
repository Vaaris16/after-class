use bevy::prelude::*;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_camera);
    }
}

fn spawn_player_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default()));
}
