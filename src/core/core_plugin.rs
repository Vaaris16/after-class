use bevy::prelude::*;

use crate::core::player_camera::player_camera_plugin::PlayerCameraPlugin;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerCameraPlugin);
    }
}
