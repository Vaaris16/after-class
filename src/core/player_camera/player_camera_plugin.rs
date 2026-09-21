use bevy::prelude::*;

use crate::GameState;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), spawn_player_camera)
            .add_systems(OnEnter(GameState::Fight), set_fight_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_player_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default(), MainCamera));
}

fn set_fight_camera(mut commands: Commands, main_camera: Single<Entity, With<MainCamera>>) {
    commands.entity(*main_camera).despawn();

    commands.spawn((Camera3d::default(), Transform::default(), MainCamera));
    println!("3d");
}
