use bevy::prelude::*;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::game_brief::materials::{
    material_image_path::MaterialImagePath, material_type::MaterialType,
};

#[derive(Resource, Default)]
pub struct Materials {
    assigned_materials: Vec<MaterialType>,
}

impl Materials {
    pub fn assign_materials(&mut self) {
        let mut rng = rand::rng();
        let mut m_types: Vec<MaterialType> = MaterialType::iter().collect();

        m_types.shuffle(&mut rng);

        self.assigned_materials = m_types.into_iter().take(5).collect();
    }
    pub fn get_card_data(&self, m_image: &MaterialImagePath) -> Vec<(&'static str, Handle<Image>)> {
        MaterialType::iter()
            .map(|m_t| (m_t.material_name(), m_image.get_material_image_path(&m_t)))
            .collect()
    }
}
