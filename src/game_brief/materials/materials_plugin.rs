use bevy::prelude::*;

use crate::game_brief::materials::material_image_path::MaterialImagePath;
use crate::game_brief::materials::materials_resource::Materials;
use crate::game_brief::materials::materials_ui::materials_ui;

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Materials>()
            .init_resource::<MaterialImagePath>()
            .add_systems(Startup, materials_ui);
    }
}
