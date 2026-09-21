use bevy::prelude::*;

use crate::GameState;

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Fight), spawn_test);
    }
}

fn spawn_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        // Create the sphere primitive and generate its mesh
        Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(5).unwrap())),
        // Apply a StandardMaterial so it responds to lighting
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.7, 0.9), // Cyan/Blueish sphere
            perceptual_roughness: 0.2,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    // 2. Spawn the Light Source (PointLight)
    commands.spawn((
        DirectionalLight {
            illuminance: 500_000.0, // High intensity for Bevy's physical light units
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 5.0, 4.0),
    ));
}
