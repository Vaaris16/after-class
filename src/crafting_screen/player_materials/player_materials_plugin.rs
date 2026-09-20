use bevy::{prelude::*, ui::debug::print_ui_layout_tree};

use crate::{
    GameState,
    core::game_fonts::fonts::GameFonts,
    crafting_screen::{
        crafting_screen_plugin::CraftingScreenSet,
        player_attributes::{self, player_attributes_plugin::PlayerAttributesPreview},
        potion::potion_plugin::Potion,
    },
    game_brief::{
        game_brief_plugin::assign_materials_to_user,
        materials::{
            material_image_path::MaterialImagePath, material_type::MaterialType,
            materials_resource::Materials,
        },
    },
};

pub struct PlayerMaterialsPlugin;

impl Plugin for PlayerMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CraftingScreen), assign_materials_to_user)
            .add_systems(
                OnEnter(GameState::CraftingScreen),
                player_materials_ui.after(assign_materials_to_user),
            )
            .add_systems(
                Update,
                (card_selection, update_number_of_selected_materials).in_set(CraftingScreenSet),
            );
    }
}

const PLAYER_MATERIALS_UI_BG_COLOR: Color = Color::hsl(250., 0.16, 0.07);
const PLAYER_MATERIALS_UI_BORDER_COLOR: Color = Color::hsl(288., 0.06, 0.15);

const PLAYER_MATERIALS_UI_TITLE: Color = Color::hsl(268., 0.45, 0.84);
const PLAYER_MATERIALS_UI_SUBTITLE: Color = Color::hsl(260., 0.06, 0.56);

#[derive(Component)]
struct NumberOfSelectedMaterialText;

const MATERIAL_UI_TEXT_COLOR: Color = Color::hsl(307., 0.14, 0.74);

fn player_materials_ui(
    mut commands: Commands,
    user_materials: Res<Materials>,
    material_image: Res<MaterialImagePath>,
    game_fonts: Res<GameFonts>,
) {
    print!("rendered");
    let user_materials = &user_materials.assigned_materials;
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    top: px(50),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: px(15),
                    ..Default::default()
                },
                children![
                    (
                        Text::new("Brew your potion"),
                        TextFont {
                            font_size: px(30).into(),
                            font: game_fonts.kaisei_decol_bold.clone().into(),
                            ..Default::default()
                        },
                        TextColor(PLAYER_MATERIALS_UI_TITLE),
                    ),
                    (
                        Text::new("Choose 4 unique materials to create a potion"),
                        TextFont {
                            font_size: px(15).into(),
                            font: game_fonts.kaisei_decol_medium.clone().into(),
                            ..Default::default()
                        },
                        TextColor(PLAYER_MATERIALS_UI_SUBTITLE),
                    )
                ],
            ));
            parent
                .spawn((
                    Node {
                        height: percent(25),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::SpaceBetween,
                        bottom: percent(3),
                        right: percent(7),
                        left: percent(7),
                        border_radius: BorderRadius::all(px(20)),
                        border: UiRect::all(px(2)),
                        padding: UiRect::all(px(15)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    BorderColor::all(PLAYER_MATERIALS_UI_BORDER_COLOR),
                    BackgroundColor(PLAYER_MATERIALS_UI_BG_COLOR),
                ))
                .with_children(|cards_grid| {
                    cards_grid.spawn((
                        Node {
                            width: percent(100),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        children![
                            (
                                Text::new("Your Materials"),
                                TextFont {
                                    font_size: px(20).into(),
                                    font: game_fonts.kaisei_decol_bold.clone().into(),
                                    ..Default::default()
                                },
                                TextColor(MATERIAL_UI_TEXT_COLOR),
                            ),
                            (
                                Text::new(""),
                                TextFont {
                                    font_size: px(20).into(),
                                    font: game_fonts.kaisei_decol_bold.clone().into(),
                                    ..Default::default()
                                },
                                TextColor(MATERIAL_UI_TEXT_COLOR),
                                NumberOfSelectedMaterialText,
                            )
                        ],
                    ));
                    cards_grid
                        .spawn((Node {
                            margin: UiRect::top(px(20)),
                            width: percent(100),
                            height: percent(100),
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::flex(1.); 5],
                            grid_template_rows: GridTrack::flex(1.),
                            column_gap: px(20),
                            row_gap: px(20),
                            ..Default::default()
                        },))
                        .with_children(|cards| {
                            for material_type in user_materials {
                                cards.spawn(material_card(
                                    material_type,
                                    &material_image,
                                    &game_fonts,
                                ));
                            }
                        });
                });
        });
}

const MATERIAL_CARD_BG_COLOR: Color = Color::hsl(300., 0.06, 0.07);

#[derive(Component)]
struct MaterialCard(MaterialType);

#[derive(Component)]
struct SelectedMaterial;

fn material_card(
    m_t: &MaterialType,
    m_img: &MaterialImagePath,
    game_fonts: &GameFonts,
) -> impl Bundle {
    let material_image = m_img.get_material_image_path(&m_t);
    (
        Node {
            height: percent(100),
            width: percent(100),
            border_radius: BorderRadius::all(px(10)),
            border: UiRect::all(px(2)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect {
                top: px(15),
                bottom: px(15),
                ..Default::default()
            },
            ..Default::default()
        },
        BorderColor::all(PLAYER_MATERIALS_UI_BORDER_COLOR),
        BackgroundColor(MATERIAL_CARD_BG_COLOR),
        MaterialCard(m_t.clone()),
        Button,
        children![
            (
                Node {
                    width: percent(90),
                    height: percent(70),
                    ..Default::default()
                },
                ImageNode {
                    image: material_image,
                    ..Default::default()
                }
            ),
            (
                Text::new(m_t.material_name()),
                TextFont {
                    font_size: px(25).into(),
                    font: game_fonts.kaisei_decol_bold.clone().into(),
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            )
        ],
    )
}

const PLAYER_MATERIALS_UI_BG_COLOR_SELECTED: Color = Color::hsl(210., 0.26, 0.7);

fn card_selection(
    mut commands: Commands,
    mut potion: ResMut<Potion>,
    material_cards: Query<
        (Entity, &mut BorderColor, &Interaction, &MaterialCard),
        (Changed<Interaction>, With<MaterialCard>),
    >,
    mut player_attributes: ResMut<PlayerAttributesPreview>,
) {
    for (card_entity, mut card_border_color, card_interaction, m_c) in material_cards {
        let material = m_c.0.material_effect();
        match card_interaction {
            Interaction::Pressed => {
                if potion.materials.contains(&m_c.0) {
                    commands.entity(card_entity).remove::<SelectedMaterial>();
                    potion.remove_material(m_c.0.clone());
                    *card_border_color = BorderColor::all(PLAYER_MATERIALS_UI_BORDER_COLOR);
                    player_attributes.health -= material.health_effect;
                    player_attributes.combat -= material.combat;
                    player_attributes.speed -= material.speed;
                } else if potion.materials.len() <= 3 {
                    commands.entity(card_entity).insert(SelectedMaterial);
                    potion.add_material(m_c.0.clone());
                    *card_border_color = BorderColor::all(PLAYER_MATERIALS_UI_BG_COLOR_SELECTED);
                    player_attributes.health += material.health_effect;
                    player_attributes.combat += material.combat;
                    player_attributes.speed += material.speed;
                }
            }
            _ => (),
        }
    }
}

fn update_number_of_selected_materials(
    user_materials: Res<Potion>,
    mut num_selected_materials: Single<&mut Text, With<NumberOfSelectedMaterialText>>,
) {
    let text = format!("{}/4 Materials", user_materials.materials.len().to_string());
    num_selected_materials.0 = text;
}
