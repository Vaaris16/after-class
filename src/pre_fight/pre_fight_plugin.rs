use std::time::Duration;

use bevy::{asset::io::AssetSource, log::tracing::Instrument, prelude::*};

use crate::{GameState, core::game_fonts::fonts::GameFonts};

pub struct PreFightPlugin;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
struct PreFightSet;

impl Plugin for PreFightPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, PreFightSet.run_if(in_state(GameState::PreFight)));
        app.insert_resource(PreFightTimer(Timer::new(
            Duration::from_secs(3),
            TimerMode::Once,
        )));
        app.add_systems(OnEnter(GameState::PreFight), (set_bg, spawn_pre_fight_ui))
            .add_systems(OnExit(GameState::PreFight), cleanup_pre_fight_ui)
            .add_systems(Update, (resize_bg, end_pre_fight).in_set(PreFightSet));
    }
}

#[derive(Resource)]
struct PreFightTimer(Timer);

#[derive(Component)]
struct PreFightBG;

fn set_bg(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Sprite {
            image: asset_server.load("night_time_castle.png"),
            ..Default::default()
        },
        Transform::default(),
        PreFightBG,
    ));
}

fn resize_bg(mut bg: Single<&mut Sprite, With<PreFightBG>>, window: Single<&Window>) {
    bg.custom_size = Some(Vec2::new(window.width(), window.height()));
}

const PRE_FIGHT_TEXT_COLOR: Color = Color::hsl(34., 0.68, 0.91);

#[derive(Component)]
struct PreFightUi;

fn spawn_pre_fight_ui(mut commands: Commands, game_fonts: Res<GameFonts>) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(25),
            ..Default::default()
        },
        PreFightUi,
        children![
            (
                Text::new("Get Ready"),
                TextFont {
                    font_size: px(100).into(),
                    font: game_fonts.kaisei_decol_bold.clone().into(),
                    ..Default::default()
                },
                TextColor(PRE_FIGHT_TEXT_COLOR),
            ),
            (
                Text::new("Its time to fight"),
                TextFont {
                    font_size: px(50).into(),
                    font: game_fonts.kaisei_decol_medium.clone().into(),
                    ..Default::default()
                },
                TextColor(PRE_FIGHT_TEXT_COLOR),
            )
        ],
    ));
}

fn end_pre_fight(
    time: Res<Time>,
    mut timer: ResMut<PreFightTimer>,
    mut state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        state.set(GameState::Fight);
    }
}

fn cleanup_pre_fight_ui(mut commands: Commands, pre_fight_ui: Single<Entity, With<PreFightUi>>) {
    commands.entity(*pre_fight_ui).despawn();
}
