use bevy::prelude::*;

use crate::game_brief::materials::material_type::MaterialType;

#[derive(Resource, Default)]
pub struct Potion {
    pub materials: Vec<MaterialType>,
}

pub struct PotionPlugin;

impl Plugin for PotionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Potion>();
    }
}

impl Potion {
    pub fn add_material(&mut self, m_t: MaterialType) {
        self.materials.push(m_t);
    }

    pub fn remove_material(&mut self, m_t: MaterialType) {
        self.materials.retain(|mt| mt != &m_t);
    }
}
