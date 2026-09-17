use strum::EnumIter;

use crate::game_brief::materials::material_effect::MaterialEffect;

#[derive(EnumIter, Debug, Clone, PartialEq)]
pub enum MaterialType {
    AetherCrystal,
    Moonstone,
    EmberCrystal,
    VoidShard,
    StormCrystal,
    FrostCrystal,
    LumenShard,
    AstralCrystal,
}

impl MaterialType {
    pub fn material_name(&self) -> &'static str {
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

    pub fn material_effect(&self) -> MaterialEffect {
        match self {
            Self::AetherCrystal => MaterialEffect {
                health_effect: 5.,
                combat: -2.,
                shield: 1.,
                ..Default::default()
            },

            Self::Moonstone => MaterialEffect {
                health_effect: 3.,
                speed: -1.,
                shield: 4.,
                ..Default::default()
            },

            Self::EmberCrystal => MaterialEffect {
                health_effect: -2.,
                combat: 6.,
                speed: 1.,
                shield: -1.,
            },

            Self::VoidShard => MaterialEffect {
                health_effect: -1.,
                combat: 4.,
                speed: 3.,
                shield: -2.,
            },

            Self::StormCrystal => MaterialEffect {
                health_effect: -2.,
                combat: 2.,
                speed: 5.,
                shield: -1.,
            },

            Self::FrostCrystal => MaterialEffect {
                health_effect: 2.,
                speed: -2.,
                shield: 5.,
                ..Default::default()
            },

            Self::LumenShard => MaterialEffect {
                health_effect: 6.,
                combat: -3.,
                shield: 2.,
                ..Default::default()
            },

            Self::AstralCrystal => MaterialEffect {
                health_effect: 2.,
                combat: 3.,
                speed: 2.,
                shield: 2.,
            },
        }
    }
}
