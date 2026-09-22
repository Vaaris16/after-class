use crate::GameState;
use bevy::{
    camera_controller::free_camera::FreeCamera, core_pipeline::tonemapping::Tonemapping,
    image::ImageFormat::Hdr, pbr::ScreenSpaceAmbientOcclusion, post_process::bloom::Bloom,
    prelude::*,
};

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

fn set_fight_camera(mut commands: Commands) {
    // uncomment the code below when done testing Fight
    // commands.entity(*main_camera).despawn();

    commands.spawn((
        Camera3d::default(),
        FreeCamera {
            sensitivity: 0.2,
            friction: 25.0,
            walk_speed: 30.0,
            run_speed: 9.0,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0).looking_to(Vec3::X, Vec3::Y),
        MainCamera,
        Tonemapping::BlenderFilmic,
        ScreenSpaceAmbientOcclusion::default(),
        Msaa::Off,
    ));
    println!("3d");
}
