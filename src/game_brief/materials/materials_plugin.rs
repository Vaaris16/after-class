use bevy::prelude::*;
use rand::RngExt;
use rand::seq::SliceRandom;
use strum::EnumIter;
use strum::IntoEnumIterator;

use crate::game_brief::materials::materials_ui::materials_ui;

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Materials>()
            .init_resource::<MaterialImagePath>()
            .add_systems(Startup, materials_ui);
    }
}

#[derive(Resource, Debug)]
pub struct Materials {
    pub aether_crystal: Material,
    pub moonstone: Material,
    pub ember_crystal: Material,
    pub void_shard: Material,
    pub storm_crystal: Material,
    pub frost_crystal: Material,
    pub lumen_shard: Material,
    pub astral_crystal: Material,
}

#[derive(Debug)]
struct Material {
    amount: i32,
    effect: MaterialEffect,
}

#[derive(Default, Debug)]
struct MaterialEffect {
    health_effect: f32,
    combat: f32,
    speed: f32,
    shield: f32,
}

#[derive(EnumIter)]
enum MaterialType {
    AetherCrystal,
    Moonstone,
    EmberCrystal,
    VoidShard,
    StormCrystal,
    FrostCrystal,
    LumenShard,
    AstralCrystal,
}

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
    fn get_material_image_path(&self, m_t: &MaterialType) -> Handle<Image> {
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

impl MaterialType {
    fn material_name(&self) -> &'static str {
        match self {
            MaterialType::AetherCrystal => "Aether Crystal",
            MaterialType::Moonstone => "Moonstone",
            MaterialType::EmberCrystal => "Ember Crystal",
            MaterialType::VoidShard => "Void Shard",
            MaterialType::StormCrystal => "Storm Crystal",
            MaterialType::FrostCrystal => "Frost Crystal",
            MaterialType::LumenShard => "Lumen Shard",
            MaterialType::AstralCrystal => "Astral Crystal",
        }
    }
}

impl Default for Materials {
    fn default() -> Self {
        Materials {
            aether_crystal: Material {
                amount: 0,
                effect: MaterialEffect {
                    health_effect: 5.,
                    combat: -2.,
                    shield: 1.,
                    ..Default::default()
                },
            },

            moonstone: Material {
                amount: 0,
                effect: MaterialEffect {
                    health_effect: 3.,
                    speed: -1.,
                    shield: 4.,
                    ..Default::default()
                },
            },

            ember_crystal: Material {
                amount: 0,
                effect: MaterialEffect {
                    health_effect: -2.,
                    combat: 6.,
                    speed: 1.,
                    shield: -1.,
                },
            },

            void_shard: Material {
                amount: 0,
                effect: MaterialEffect {
                    health_effect: -1.,
                    combat: 4.,
                    speed: 3.,
                    shield: -2.,
                },
            },

            storm_crystal: Material {
                amount: 0,
                effect: MaterialEffect {
                    health_effect: -2.,
                    combat: 2.,
                    speed: 5.,
                    shield: -1.,
                },
            },

            frost_crystal: Material {
                amount: 0,
                effect: MaterialEffect {
                    health_effect: 2.,
                    speed: -2.,
                    shield: 5.,
                    ..Default::default()
                },
            },

            lumen_shard: Material {
                amount: 0,
                effect: MaterialEffect {
                    health_effect: 6.,
                    combat: -3.,
                    shield: 2.,
                    ..Default::default()
                },
            },

            astral_crystal: Material {
                amount: 0,
                effect: MaterialEffect {
                    health_effect: 2.,
                    combat: 3.,
                    speed: 2.,
                    shield: 2.,
                },
            },
        }
    }
}

impl Materials {
    fn increment_material_amout(&mut self, material_type: &MaterialType, amount: i32) {
        match material_type {
            MaterialType::AetherCrystal => self.aether_crystal.amount += amount,
            MaterialType::Moonstone => self.moonstone.amount += amount,
            MaterialType::EmberCrystal => self.ember_crystal.amount += amount,
            MaterialType::VoidShard => self.void_shard.amount += amount,
            MaterialType::StormCrystal => self.storm_crystal.amount += amount,
            MaterialType::FrostCrystal => self.frost_crystal.amount += amount,
            MaterialType::LumenShard => self.lumen_shard.amount += amount,
            MaterialType::AstralCrystal => self.astral_crystal.amount += amount,
        }
    }
    pub fn assign_materials(&mut self) {
        let mut rng = rand::rng();
        let mut m_types: Vec<MaterialType> = MaterialType::iter().collect();

        m_types.shuffle(&mut rng);

        for material in m_types.iter().take(5) {
            let amount = rng.random_range(1..6);
            self.increment_material_amout(material, amount);
        }
    }
    fn get_material_data(&self, m_t: &MaterialType) -> &Material {
        match m_t {
            MaterialType::AetherCrystal => &self.aether_crystal,
            MaterialType::Moonstone => &self.moonstone,
            MaterialType::EmberCrystal => &self.ember_crystal,
            MaterialType::VoidShard => &self.void_shard,
            MaterialType::StormCrystal => &self.storm_crystal,
            MaterialType::FrostCrystal => &self.frost_crystal,
            MaterialType::LumenShard => &self.lumen_shard,
            MaterialType::AstralCrystal => &self.astral_crystal,
        }
    }
    pub fn get_card_data(
        &self,
        m_image: &MaterialImagePath,
    ) -> Vec<(&'static str, Handle<Image>, i32)> {
        MaterialType::iter()
            .map(|m_t| {
                (
                    m_t.material_name(),
                    m_image.get_material_image_path(&m_t),
                    self.get_material_data(&m_t).amount,
                )
            })
            .collect()
    }
}
