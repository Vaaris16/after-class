use bevy::{camera_controller::free_camera::FreeCameraPlugin, prelude::*, window::WindowMode};

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
    SplashScreen,
    GameBrief,
    CraftingScreen,
    PreFight,
    #[default]
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
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(0.6, 0.7, 0.8),
            brightness: 300.0,
            affects_lightmapped_meshes: true,
        })
        .add_plugins((
            SplashScreenPlugin,
            GameBriefplugin,
            CraftingScreenPlugin,
            PreFightPlugin,
            FightPlugin,
            CorePlugin,
            FreeCameraPlugin,
        ))
        .init_state::<GameState>()
        .run();
}
