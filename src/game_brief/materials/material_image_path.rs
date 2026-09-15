use bevy::prelude::*;

use crate::game_brief::materials::material_type::MaterialType;

#[derive(Resource)]
pub struct MaterialImagePath {
    pub aether_crystal: Handle<Image>,
    pub moonstone: Handle<Image>,
    pub ember_crystal: Handle<Image>,
    pub void_shard: Handle<Image>,
    pub storm_crystal: Handle<Image>,
    pub frost_crystal: Handle<Image>,
    pub lumen_shard: Handle<Image>,
    pub astral_crystal: Handle<Image>,
}

impl FromWorld for MaterialImagePath {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        MaterialImagePath {
            aether_crystal: asset_server.load("materials/aether_crystal.png"),
            moonstone: asset_server.load("materials/moonstone.png"),
            ember_crystal: asset_server.load("materials/ember_crystal.png"),
            void_shard: asset_server.load("materials/void_shard.png"),
            storm_crystal: asset_server.load("materials/storm_crystal.png"),
            frost_crystal: asset_server.load("materials/frost_crystal.png"),
            lumen_shard: asset_server.load("materials/lumen_shard.png"),
            astral_crystal: asset_server.load("materials/astral_crystal.png"),
        }
    }
}

impl MaterialImagePath {
    pub fn get_material_image_path(&self, m_t: &MaterialType) -> Handle<Image> {
        match m_t {
            MaterialType::AetherCrystal => self.aether_crystal.clone(),
            MaterialType::Moonstone => self.moonstone.clone(),
            MaterialType::EmberCrystal => self.ember_crystal.clone(),
            MaterialType::VoidShard => self.void_shard.clone(),
            MaterialType::StormCrystal => self.storm_crystal.clone(),
            MaterialType::FrostCrystal => self.frost_crystal.clone(),
            MaterialType::LumenShard => self.lumen_shard.clone(),
            MaterialType::AstralCrystal => self.astral_crystal.clone(),
        }
    }
}
