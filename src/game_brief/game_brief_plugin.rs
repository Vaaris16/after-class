use bevy::prelude::*;

use crate::{
    GameState,
    game_brief::{
        dialog_box::dialog_box_plugin::{DialogBox, DialogBoxPlugin},
        materials::{
            materials_plugin::MaterialsPlugin, materials_resource::Materials,
            materials_ui::MaterialUi,
        },
    },
};

pub struct GameBriefplugin;

impl Plugin for GameBriefplugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameBrief), set_background)
            //            .add_systems(Startup, assign_materials_to_user)
            .add_systems(Update, resize_bg)
            .add_plugins((DialogBoxPlugin, MaterialsPlugin))
            .add_systems(OnExit(GameState::GameBrief), cleanup_gamebreif_ui);
    }
}

#[derive(Component)]
struct GameBriefBackground;

fn set_background(mut commands: Commands, asset_server: Res<AssetServer>, window: Single<&Window>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("game_brief_background.png"),
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GameBriefBackground,
    ));
}

fn resize_bg(window: Single<&Window>, mut bg: Single<&mut Sprite, With<GameBriefBackground>>) {
    bg.custom_size = Some(Vec2::new(window.width(), window.height()));
}

fn cleanup_gamebreif_ui(
    dialog_ui: Single<Entity, With<DialogBox>>,
    material_ui: Single<Entity, With<MaterialUi>>,
    mut commands: Commands,
) {
    commands.entity(*dialog_ui).despawn();
    commands.entity(*material_ui).despawn();
}

pub fn assign_materials_to_user(mut materials: ResMut<Materials>) {
    materials.assign_materials();
}
