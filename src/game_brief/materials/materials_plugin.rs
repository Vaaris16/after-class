use bevy::prelude::*;

use crate::GameState;

use crate::game_brief::materials::{
    material_image_path::MaterialImagePath, materials_resource::Materials,
    materials_ui::materials_ui,
};

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Materials>()
            .init_resource::<MaterialImagePath>()
            .add_systems(OnEnter(GameState::GameBrief), materials_ui);
    }
}
