use bevy::prelude::*;

use crate::{
    GameState::{self, SplashScreen},
    core::game_fonts::fonts::GameFonts,
};

pub struct SplashScreenPlugin;

#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
struct SplashScreenSet;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            SplashScreenSet.run_if(in_state(GameState::SplashScreen)),
        );
        app.add_systems(
            OnEnter(GameState::SplashScreen),
            (set_splash_background, spawn_splash_screen),
        )
        .add_systems(Update, start_button_interactions.in_set(SplashScreenSet))
        .add_systems(OnExit(GameState::SplashScreen), cleanup_splashscreen);
    }
}

#[derive(Component)]
struct SplashScreenComponent;

fn spawn_splash_screen(mut commands: Commands, game_fonts: Res<GameFonts>) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        SplashScreenComponent,
        ZIndex(1),
        children![(
            Node {
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(20),
                margin: UiRect::left(px(20)),
                ..Default::default()
            },
            children![splash_title(&game_fonts), start_button(&game_fonts)],
        )],
    ));
}

const SPLASH_TITLE_COLOR: Color = Color::hsl(30., 0.65, 0.96);

fn splash_title(game_fonts: &GameFonts) -> impl Bundle {
    (
        Text::new("After Class"),
        TextFont {
            font_size: px(100).into(),
            font: game_fonts.lora_font_bold.clone().into(),
            ..Default::default()
        },
        TextColor(SPLASH_TITLE_COLOR),
    )
}

fn set_splash_background(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        SplashScreenComponent,
        Node {
            width: percent(100),
            height: percent(100),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        ImageNode {
            image: asset_server.load("splash_screen_bg.png"),
            image_mode: NodeImageMode::Stretch,
            ..Default::default()
        },
        ZIndex(0),
    ));
}

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct StartButtonText;

const START_BUTTON_BACKGROUND_COLOR: Color = Color::hsl(244., 0.33, 0.25);
const START_BUTTON_BORDER_COLOR: Color = Color::hsl(267., 0.56, 0.77);
const START_BUTTON_TEXT_COLOR: Color = Color::hsl(300., 0.08, 0.95);

fn start_button(game_fonts: &GameFonts) -> impl Bundle {
    (
        Button,
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::MAX,
            padding: UiRect {
                top: px(10),
                bottom: px(10),
                right: px(90),
                left: px(90),
            },
            border: UiRect::all(px(3)),
            ..Default::default()
        },
        StartButton,
        BackgroundColor(START_BUTTON_BACKGROUND_COLOR),
        BorderColor::all(START_BUTTON_BORDER_COLOR),
        children![(
            StartButtonText,
            Text::new("Start"),
            TextFont {
                font_size: px(27).into(),
                font: game_fonts.kaisei_decol_bold.clone().into(),
                ..Default::default()
            },
            TextColor(START_BUTTON_TEXT_COLOR)
        )],
    )
}

fn start_button_interactions(
    button: Query<(&Interaction, &mut BackgroundColor), (With<StartButton>, Changed<Interaction>)>,
    mut button_text: Single<&mut TextColor, With<StartButtonText>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (button_interactions, mut button_bg) in button {
        match button_interactions {
            Interaction::Hovered => {
                button_bg.0 = Color::WHITE;
                button_text.0 = Color::BLACK;
            }
            Interaction::None => {
                button_bg.0 = START_BUTTON_BACKGROUND_COLOR;
                button_text.0 = START_BUTTON_TEXT_COLOR;
            }
            Interaction::Pressed => {
                game_state.set(GameState::GameBrief);
            }
        }
    }
}

fn cleanup_splashscreen(
    mut commands: Commands,
    splash_screen: Query<Entity, With<SplashScreenComponent>>,
) {
    for splash_entity in splash_screen {
        commands.entity(splash_entity).despawn();
    }
}
