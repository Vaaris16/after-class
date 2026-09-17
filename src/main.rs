use bevy::{prelude::*, window::WindowMode};

use crate::{
    core::core_plugin::CorePlugin, crafting_screen::crafting_screen_plugin::CraftingScreenPlugin,
    game_brief::game_brief_plugin::GameBriefplugin,
    splashscreen::splashscreen_plugin::SplashScreenPlugin,
};

mod core;
mod crafting_screen;
mod game_brief;
mod splashscreen;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
    SplashScreen,
    GameBrief,
    #[default]
    CraftingScreen,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            SplashScreenPlugin,
            GameBriefplugin,
            CraftingScreenPlugin,
            CorePlugin,
        ))
        .init_state::<GameState>()
        .run();
}
