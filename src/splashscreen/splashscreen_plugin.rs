use bevy::prelude::*;

use crate::GameState::{self, SplashScreen};

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

fn spawn_splash_screen(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
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
            children![splash_title(), start_button()],
        )],
    ));
}

fn splash_title() -> impl Bundle {
    (
        Text::new("After Class"),
        TextFont {
            font_size: px(90).into(),
            ..Default::default()
        },
        TextColor(Color::BLACK),
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

fn start_button() -> impl Bundle {
    (
        Button,
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(px(20)),
            padding: UiRect::all(px(15)),
            ..Default::default()
        },
        StartButton,
        BackgroundColor(Color::BLACK),
        children![(
            StartButtonText,
            Text::new("Start"),
            TextFont {
                font_size: px(33).into(),
                ..default()
            },
            TextColor(Color::WHITE)
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
                button_bg.0 = Color::BLACK;
                button_text.0 = Color::WHITE;
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
