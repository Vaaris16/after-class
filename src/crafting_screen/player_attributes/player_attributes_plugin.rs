use bevy::prelude::*;

use crate::{
    GameState,
    crafting_screen::{
        crafting_screen_plugin::CraftingScreenSet,
        player_attributes::player_attributes_ui::{
            set_attribute_values, spawn_player_attributes_ui,
        },
    },
};

pub struct PlayerAttributePlugin;

impl Plugin for PlayerAttributePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAttributes>()
            .init_resource::<PlayerAttributesPreview>()
            .add_systems(
                OnEnter(GameState::CraftingScreen),
                spawn_player_attributes_ui,
            )
            .add_systems(Update, set_attribute_values.in_set(CraftingScreenSet));
    }
}

#[derive(Resource)]
pub struct PlayerAttributesPreview {
    pub health: f32,
    pub combat: f32,
    pub speed: f32,
}

impl Default for PlayerAttributesPreview {
    fn default() -> Self {
        Self {
            health: 100.,
            combat: 20.,
            speed: 20.,
        }
    }
}

#[derive(Resource)]
pub struct PlayerAttributes {
    pub health: f32,
    pub combat: f32,
    pub speed: f32,
}

impl Default for PlayerAttributes {
    fn default() -> Self {
        Self {
            health: 100.,
            combat: 20.,
            speed: 20.,
        }
    }
}

impl PlayerAttributes {
    pub fn apply_effects(&mut self, health: f32, combat: f32, speed: f32) {
        self.health = health;
        self.combat = combat;
        self.speed = speed;
    }
}
