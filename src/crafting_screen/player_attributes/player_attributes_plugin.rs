use bevy::prelude::*;

use crate::{
    crafting_screen::player_attributes::player_attributes_ui::{
        set_attribute_values, spawn_player_attributes_ui,
    },
    game_brief::materials::material_type::MaterialType,
};

pub struct PlayerAttributePlugin;

impl Plugin for PlayerAttributePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAttributes>()
            .init_resource::<PlayerAttributesPreview>()
            .add_systems(Startup, spawn_player_attributes_ui)
            .add_systems(Update, set_attribute_values);
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
    fn apply_effect(&mut self, m_t: MaterialType) {
        self.health += m_t.material_effect().health_effect;
        self.combat += m_t.material_effect().combat;
        self.speed += m_t.material_effect().speed;
    }
}
