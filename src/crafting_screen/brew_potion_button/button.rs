use bevy::prelude::*;

use crate::{
    GameState,
    core::game_fonts::fonts::GameFonts,
    crafting_screen::{
        player_attributes::player_attributes_plugin::{PlayerAttributes, PlayerAttributesPreview},
        potion::potion_plugin::Potion,
    },
};

const BUTTON_SIZE: Vec2 = Vec2::new(350., 75.);

const BUTTON_BORDER_COLOR: Color = Color::hsl(280., 0.08, 0.22);
const BUTTON_BACKGROUND_COLOR: Color = Color::hsl(249., 0.1, 0.13);

const BUTTON_BORDER_COLOR_SELECTED: Color = Color::hsl(33., 0.36, 0.60);
const BUTTON_TEXT_COLOR: Color = Color::hsl(280., 0.06, 0.47);
const BUTTON_TEXT_COLOR_SELECTED: Color = Color::hsl(27., 0.51, 0.71);

pub struct BrewPotionPlugin;

impl Plugin for BrewPotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CraftingScreen), spawn_brew_button)
            .add_systems(Update, brew_button_interaction);
    }
}

#[derive(Component)]
pub struct BrewPotionButton;

#[derive(Component)]
struct BrewPotionButtonText;

pub fn spawn_brew_button(
    mut commands: Commands,
    window: Single<&Window>,
    game_fonts: Res<GameFonts>,
) {
    commands.spawn((
        Button,
        Node {
            width: px(BUTTON_SIZE.x),
            height: px(BUTTON_SIZE.y),
            position_type: PositionType::Absolute,
            top: px(window.height() / 2. - BUTTON_SIZE.y / 2. + 140.),
            right: px(window.width() / 2. - BUTTON_SIZE.x / 2.),
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(px(10)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BorderColor::all(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BACKGROUND_COLOR),
        BrewPotionButton,
        children![(
            BrewPotionButtonText,
            Text::new("Brew Potion"),
            TextFont {
                font_size: px(20).into(),
                font: game_fonts.kaisei_decol_bold.clone().into(),
                ..Default::default()
            },
            TextColor(BUTTON_TEXT_COLOR),
        )],
    ));
}

fn brew_button_interaction(
    button: Single<
        (&Interaction, &mut BorderColor),
        (With<BrewPotionButton>, Changed<Interaction>),
    >,
    mut button_text: Single<&mut TextColor, With<BrewPotionButtonText>>,
    mut player_attributes: ResMut<PlayerAttributes>,
    player_attributes_preview: Res<PlayerAttributesPreview>,
    potion: Res<Potion>,
    mut state: ResMut<NextState<GameState>>,
) {
    let (interaction_button, mut border_color) = button.into_inner();
    let press_button = if potion.materials.len() == 4 {
        true
    } else {
        false
    };
    match interaction_button {
        Interaction::Pressed => {
            if press_button {
                player_attributes.apply_effects(
                    player_attributes_preview.health,
                    player_attributes_preview.combat,
                    player_attributes_preview.speed,
                );
            }
            state.set(GameState::PreFight);
        }
        Interaction::Hovered => {
            if press_button {
                button_text.0 = BUTTON_TEXT_COLOR_SELECTED;
                *border_color = BorderColor::all(BUTTON_BORDER_COLOR_SELECTED);
            }
        }
        Interaction::None => {
            button_text.0 = BUTTON_TEXT_COLOR;
            *border_color = BorderColor::all(BUTTON_BORDER_COLOR);
        }
    }
}
