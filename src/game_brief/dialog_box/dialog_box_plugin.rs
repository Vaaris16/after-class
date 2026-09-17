use bevy::{ecs::system::IntoResult, prelude::*, ui::debug::print_ui_layout_tree};

use crate::{
    GameState, core::game_fonts::fonts::GameFonts,
    game_brief::materials::materials_resource::Materials,
};

pub struct DialogBoxPlugin;

impl Plugin for DialogBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameBrief), spawn_dialog_box);
    }
}

#[derive(Component)]
pub struct DialogBox;

fn spawn_dialog_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_fonts: Res<GameFonts>,
) {
    commands.spawn((
        DialogBox,
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

const TEXT_COLOR_GAME_BREIF: Color = Color::hsl(247.5, 0.706, 0.20);

fn dialog_texts(game_fonts: &GameFonts) -> impl Bundle {
    children![
        (text_helper_func(
            "Welcome Apprentice,",
            px(30),
            game_fonts.kaisei_decol_bold.clone()
        )),
        (
            Node {
                margin: UiRect {
                    top: px(10),
                    bottom: px(10),
                    ..Default::default()
                },
                ..Default::default()
            },
            text_helper_func(
                "Your journey begins here",
                px(18),
                game_fonts.kaisei_decol_bold.clone()
            )
        ),
        (text_helper_func(
            "I've given you a collection of magical materials. Use them to brew a potion to enhance your player and try to defeat the night zombies.",
            px(18),
            game_fonts.kaisei_decol_bold.clone()
        )),
        (
            Node {
                margin: UiRect::top(px(20)),
                ..Default::default()
            },
            text_helper_func(
                "Press space to continue",
                px(18),
                game_fonts.kaisei_decol_bold.clone()
            )
        )
    ]
}

fn text_helper_func(text: &str, font_s: Val, text_font: Handle<Font>) -> impl Bundle {
    (
        Text::new(text),
        TextLayout::new(Justify::Left, LineBreak::WordBoundary),
        TextFont {
            font_size: font_s.into(),
            font: text_font.into(),
            ..Default::default()
        },
        TextColor(TEXT_COLOR_GAME_BREIF),
    )
}
