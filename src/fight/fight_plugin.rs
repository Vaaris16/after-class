use bevy::{gltf::GltfSceneName, prelude::*, ui::debug::print_ui_layout_tree};

use crate::GameState;

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Fight), spawn_forest);
    }
}

fn spawn_forest(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        WorldAssetRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("3d models/forest_arena_model/forest_arena.glb"),
        )),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 1., 0.0),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 30_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
