use bevy::{ecs::system::IntoResult, prelude::*};

use crate::{GameState, core::game_fonts::fonts::GameFonts};

pub struct DialogBoxPlugin;

impl Plugin for DialogBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameBrief), spawn_dialog_box);
    }
}

fn spawn_dialog_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_fonts: Res<GameFonts>,
) {
    commands.spawn((
        Node {
            width: px(450),
            height: px(250),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            left: px(20),
            top: px(50),
            ..Default::default()
        },
        ImageNode {
            image: asset_server.load("dialog_box.png"),
            image_mode: NodeImageMode::Stretch,
            ..Default::default()
        },
        children![(
            Node {
                width: percent(90),
                height: percent(85),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(px(5)),
                ..Default::default()
            },
            dialog_texts(&game_fonts)
        )],
    ));
}

fn dialog_texts(game_fonts: &GameFonts) -> impl Bundle {
    children![
        (
            Text::new("Welcome"),
            TextLayout::new(Justify::Left, LineBreak::WordBoundary),
            TextFont {
                font_size: px(30).into(),
                font: game_fonts.kaisei_decol_bold.clone().into(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ),
        (
            Node {
                margin: UiRect {
                    top: px(10),
                    bottom: px(10),
                    ..Default::default()
                },
                ..Default::default()
            },
            Text::new("Your journey begins here"),
            TextLayout::new(Justify::Left, LineBreak::WordBoundary),
            TextFont {
                font_size: px(18).into(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ),
        (
            Text::new(
                "I've given you a collection of magical materials. Use them to brew a potion to enhance your player and try to defeat the night zombies.",
            ),
            TextLayout::new(Justify::Left, LineBreak::WordBoundary),
            TextFont {
                font_size: px(18).into(),
                font: game_fonts.kaisei_decol_bold.clone().into(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ),
        (
            Node {
                margin: UiRect::top(px(20)),
                ..Default::default()
            },
            Text::new("Press space to continue"),
            TextLayout::new(Justify::Left, LineBreak::WordBoundary),
            TextFont {
                font_size: px(18).into(),
                font: game_fonts.kaisei_decol_bold.clone().into(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        )
    ]
}
