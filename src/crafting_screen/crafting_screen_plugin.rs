use bevy::prelude::*;

use crate::{
    GameState,
    crafting_screen::{
        player_materials::player_materials_plugin::PlayerMaterialsPlugin,
        potion::potion_plugin::PotionPlugin,
    },
};

#[derive(SystemSet, Hash, PartialEq, Eq, Debug, Clone)]
pub struct CraftingScreenSet;

pub struct CraftingScreenPlugin;

impl Plugin for CraftingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            CraftingScreenSet.run_if(in_state(GameState::CraftingScreen)),
        );
        app.add_systems(OnEnter(GameState::CraftingScreen), set_bg)
            .add_systems(Update, resize_bg.in_set(CraftingScreenSet))
            .add_plugins((PotionPlugin, PlayerMaterialsPlugin));
    }
}

#[derive(Component)]
struct CraftingSceenBackground;

fn set_bg(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("tmp_crafting_screen_bg.png"),
            ..Default::default()
        },
        Transform::default(),
        CraftingSceenBackground,
    ));
}

fn resize_bg(window: Single<&Window>, mut bg: Single<&mut Sprite, With<CraftingSceenBackground>>) {
    bg.custom_size = Some(Vec2::new(window.width(), window.height()));
}
