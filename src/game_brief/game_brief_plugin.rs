use bevy::prelude::*;

use crate::{GameState, game_brief::dialog_box::dialog_box_plugin::DialogBoxPlugin};

pub struct GameBriefplugin;

impl Plugin for GameBriefplugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameBrief), set_background)
            .add_plugins((DialogBoxPlugin));
    }
}

fn set_background(mut commands: Commands, asset_server: Res<AssetServer>, window: Single<&Window>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("game_brief_background.png"),
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
