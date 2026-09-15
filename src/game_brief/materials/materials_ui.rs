use bevy::prelude::*;

use crate::{
    core::game_fonts::fonts::GameFonts,
    game_brief::materials::materials_plugin::{MaterialImagePath, Materials},
};

const BACKGROUND_START: Color = Color::hsl(229., 0.64, 0.09);
const BACKGROUND_END: Color = Color::hsl(235., 0.64, 0.14);

const MATERIALS_UI_BORDER_COLOR: Color = Color::hsl(262., 0.9, 0.61);

pub fn materials_ui(
    mut commands: Commands,
    materials: Res<Materials>,
    m_image: Res<MaterialImagePath>,
    game_fonts: Res<GameFonts>,
) {
    let cards_data = materials.get_card_data(&m_image);
    commands
        .spawn((
            Node {
                width: px(550),
                height: px(375),
                position_type: PositionType::Absolute,
                top: px(20),
                right: px(40),
                border_radius: BorderRadius::all(px(20)),
                border: UiRect::all(px(2)),
                padding: UiRect::all(px(13)),
                ..Default::default()
            },
            BorderColor::all(MATERIALS_UI_BORDER_COLOR),
            BackgroundGradient::from(LinearGradient::to_top_right(vec![
                ColorStop::auto(BACKGROUND_START),
                ColorStop::auto(BACKGROUND_END),
            ])),
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    width: percent(100),
                    height: percent(100),
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::flex(1.); 4],
                    grid_template_rows: vec![GridTrack::flex(1.); 2],
                    row_gap: px(10),
                    column_gap: px(10),
                    ..Default::default()
                },))
                .with_children(|cards| {
                    for (name, material_img, material_amount) in cards_data {
                        cards.spawn(material_individual_card(
                            name,
                            material_img,
                            material_amount,
                            &game_fonts.inter_bold,
                        ));
                    }
                });
        });
}

const MATERIAL_CARDS_BACKGROUND: Color = Color::hsl(231.2, 0.63, 0.11);

fn material_individual_card(
    material_name: &'static str,
    material_image: Handle<Image>,
    material_amount: i32,
    inter_bold: &Handle<Font>,
) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(px(20)),
            padding: UiRect {
                top: px(15),
                bottom: px(15),
                right: px(5),
                left: px(5),
            },
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            row_gap: px(5),
            ..Default::default()
        },
        BorderColor::all(MATERIALS_UI_BORDER_COLOR),
        BackgroundColor(MATERIAL_CARDS_BACKGROUND),
        children![
            (
                Node {
                    width: percent(90),
                    height: percent(55),
                    ..Default::default()
                },
                ImageNode {
                    image: material_image,
                    ..Default::default()
                }
            ),
            (
                Text::new(material_name),
                TextFont {
                    font_size: px(13).into(),
                    font: inter_bold.into(),
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Text::new(material_amount.to_string()),
                TextFont {
                    font_size: px(13).into(),
                    font: inter_bold.into(),
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            )
        ],
    )
}
