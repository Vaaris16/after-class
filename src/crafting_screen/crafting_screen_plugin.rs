use bevy::prelude::*;

use crate::{
    GameState,
    crafting_screen::{
        brew_potion_button::button::{BrewPotionButton, BrewPotionPlugin, spawn_brew_button},
        player_attributes::{
            player_attributes_plugin::PlayerAttributePlugin,
            player_attributes_ui::PlayerAttributesUi,
        },
        player_materials::player_materials_plugin::{PlayerMaterialsPlugin, PlayerMaterialsUi},
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
            .add_systems(OnExit(GameState::CraftingScreen), cleanup_crafting_screen)
            .add_systems(Update, resize_bg.in_set(CraftingScreenSet))
            .add_plugins((
                PotionPlugin,
                PlayerMaterialsPlugin,
                PlayerAttributePlugin,
                BrewPotionPlugin,
            ));
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

fn cleanup_crafting_screen(
    mut commands: Commands,
    brew_potion_button: Single<Entity, With<BrewPotionButton>>,
    player_attributes_ui: Single<Entity, With<PlayerAttributesUi>>,
    player_material_ui: Single<Entity, With<PlayerMaterialsUi>>,
) {
    commands.entity(*brew_potion_button).despawn();
    commands.entity(*player_attributes_ui).despawn();
    commands.entity(*player_material_ui).despawn();
}
