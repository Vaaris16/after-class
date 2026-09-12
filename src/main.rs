use bevy::prelude::*;

use crate::{core::core_plugin::CorePlugin, splashscreen::splashscreen_plugin::SplashScreenPlugin};

mod core;
mod splashscreen;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
    #[default]
    SplashScreen,
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
        .add_plugins((SplashScreenPlugin, CorePlugin))
        .init_state::<GameState>()
        .run();
}
