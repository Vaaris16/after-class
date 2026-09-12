use bevy::prelude::*;

use crate::{
    core::core_plugin::CorePlugin, game_brief::game_brief_plugin::GameBriefplugin,
    splashscreen::splashscreen_plugin::SplashScreenPlugin,
};

mod core;
mod game_brief;
mod splashscreen;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
    SplashScreen,
    #[default]
    GameBrief,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((SplashScreenPlugin, GameBriefplugin, CorePlugin))
        .init_state::<GameState>()
        .run();
}
