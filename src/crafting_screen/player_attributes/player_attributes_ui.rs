use bevy::prelude::*;

use crate::{
    GameState, core::game_fonts::fonts::GameFonts,
    crafting_screen::player_attributes::player_attributes_plugin::PlayerAttributesPreview,
};

const PLAYER_ATTRIBUTES_COLOR: Color = Color::hsl(22., 0.88, 0.08);

#[derive(Component)]
pub struct PlayerAttributesUi;

pub fn spawn_player_attributes_ui(
    game_fonts: Res<GameFonts>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Node {
            width: px(350),
            height: px(400),
            position_type: PositionType::Absolute,
            top: px(150),
            left: px(50),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        ImageNode {
            image: asset_server.load("torn_paper.png"),
            ..Default::default()
        },
        PlayerAttributesUi,
        children![
            (
                Node {
                    margin: UiRect::top(px(30)),
                    ..Default::default()
                },
                Text::new("Your Stats"),
                TextFont {
                    font_size: px(40).into(),
                    font: game_fonts.kaisei_decol_bold.clone().into(),
                    ..Default::default()
                },
                TextColor(PLAYER_ATTRIBUTES_COLOR),
            ),
            (
                Node {
                    margin: UiRect::top(px(30)),
                    width: percent(75),
                    height: percent(50),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                children![
                    health_attribute(&game_fonts),
                    combat_attribute(&game_fonts),
                    speed_attribute(&game_fonts),
                ],
            ),
        ],
    ));
}

const ONE_THIRD: f32 = 100.0 / 3.0;

#[derive(Component)]
pub struct HealthAttributeValue;

fn health_attribute(game_fonts: &GameFonts) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(ONE_THIRD),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        children![
            (
                Text::new("Health"),
                TextFont {
                    font_size: px(35).into(),
                    font: game_fonts.kaisei_decol_bold.clone().into(),
                    ..Default::default()
                },
                TextColor(PLAYER_ATTRIBUTES_COLOR),
            ),
            (
                Text::new("121"),
                TextFont {
                    font_size: px(35).into(),
                    font: game_fonts.kaisei_decol_bold.clone().into(),
                    ..Default::default()
                },
                TextColor(PLAYER_ATTRIBUTES_COLOR),
                HealthAttributeValue,
            ),
        ],
    )
}

#[derive(Component)]
pub struct CombatAttributeValue;

fn combat_attribute(game_fonts: &GameFonts) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(ONE_THIRD),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        children![
            (
                Text::new("combat"),
                TextFont {
                    font_size: px(35).into(),
                    font: game_fonts.kaisei_decol_bold.clone().into(),
                    ..Default::default()
                },
                TextColor(PLAYER_ATTRIBUTES_COLOR),
            ),
            (
                Text::new("121"),
                TextFont {
                    font_size: px(35).into(),
                    font: game_fonts.kaisei_decol_bold.clone().into(),
                    ..Default::default()
                },
                TextColor(PLAYER_ATTRIBUTES_COLOR),
                CombatAttributeValue
            ),
        ],
    )
}

#[derive(Component)]
pub struct SpeedAttributeValue;

fn speed_attribute(game_fonts: &GameFonts) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(ONE_THIRD),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        children![
            (
                Text::new("Speed"),
                TextFont {
                    font_size: px(35).into(),
                    font: game_fonts.kaisei_decol_bold.clone().into(),
                    ..Default::default()
                },
                TextColor(PLAYER_ATTRIBUTES_COLOR),
            ),
            (
                Text::new("121"),
                TextFont {
                    font_size: px(35).into(),
                    font: game_fonts.kaisei_decol_bold.clone().into(),
                    ..Default::default()
                },
                TextColor(PLAYER_ATTRIBUTES_COLOR),
                SpeedAttributeValue
            ),
        ],
    )
}

pub fn set_attribute_values(
    mut health_attribute_value: Single<
        &mut Text,
        (
            With<HealthAttributeValue>,
            Without<CombatAttributeValue>,
            Without<SpeedAttributeValue>,
        ),
    >,
    mut combat_attribute_value: Single<
        &mut Text,
        (
            With<CombatAttributeValue>,
            Without<HealthAttributeValue>,
            Without<SpeedAttributeValue>,
        ),
    >,
    mut speed_attribute_value: Single<
        &mut Text,
        (
            With<SpeedAttributeValue>,
            Without<HealthAttributeValue>,
            Without<CombatAttributeValue>,
        ),
    >,
    preview_player_attributes: Res<PlayerAttributesPreview>,
) {
    health_attribute_value.0 = preview_player_attributes.health.to_string();
    combat_attribute_value.0 = preview_player_attributes.combat.to_string();
    speed_attribute_value.0 = preview_player_attributes.speed.to_string();
}
