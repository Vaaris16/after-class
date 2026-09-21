use bevy::{prelude::*, window::WindowMode};

use crate::{
    core::core_plugin::CorePlugin, crafting_screen::crafting_screen_plugin::CraftingScreenPlugin,
    fight::fight_plugin::FightPlugin, game_brief::game_brief_plugin::GameBriefplugin,
    pre_fight::pre_fight_plugin::PreFightPlugin,
    splashscreen::splashscreen_plugin::SplashScreenPlugin,
};

mod core;
mod crafting_screen;
mod fight;
mod game_brief;
mod pre_fight;
mod splashscreen;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
    #[default]
    SplashScreen,
    GameBrief,
    CraftingScreen,
    PreFight,
    Fight,
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
            PreFightPlugin,
            FightPlugin,
            CorePlugin,
        ))
        .init_state::<GameState>()
        .run();
}
