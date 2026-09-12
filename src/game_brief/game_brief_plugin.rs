use bevy::prelude::*;

use crate::GameState;

pub struct GameBriefplugin;

impl Plugin for GameBriefplugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameBrief), set_background)
            .add_systems(OnEnter(GameState::GameBrief), spawn_wizard);
    }
}

fn set_background(mut commands: Commands, asset_server: Res<AssetServer>, window: Single<&Window>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("game_brief_background.png"),
            // Optional: fit to window dimensions if needed
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..Default::default()
        },
        // In 2D world space, depth is controlled by Z coordinate in Transform
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn spawn_wizard(mut commands: Commands, asset_server: Res<AssetServer>, window: Single<&Window>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("wizard_mentor.png"),
            custom_size: Some(Vec2::splat(550.)),
            ..Default::default()
        },
        Transform::from_xyz(0., -window.height() / 2. + 250., 1.),
        ZIndex(10),
    ));
}
