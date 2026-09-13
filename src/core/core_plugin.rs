use bevy::prelude::*;

use crate::core::{
    game_fonts::fonts::GameFonts, player_camera::player_camera_plugin::PlayerCameraPlugin,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameFonts>()
            .add_plugins(PlayerCameraPlugin);
    }
}
