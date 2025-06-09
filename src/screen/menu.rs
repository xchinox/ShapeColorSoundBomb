use crate::{
    audio_server::components::{EffectName, PlaySoundEffectEvent, SfxSettings},
    ui_theme::UiTheme,
};

use super::components::ScreenState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui_root)
            .add_systems(OnEnter(ScreenState::Menu), setup_menu)
            .add_systems(
                Update,
                menu_button_system.run_if(in_state(ScreenState::Menu)),
            )
            .add_systems(OnExit(ScreenState::Menu), deconstruct_main_menu);
    }
}

#[derive(Component)]
pub struct UiRoot;

//Marker component to differentiate button types on interaction
#[derive(Component, Debug)]
pub enum MenuButtonType {
    NewGame,
    Options,
    Credits,
    Exit,
}
fn menu_button_system(
    mut interaction_q: Query<
        (&mut Interaction, &mut BackgroundColor, &MenuButtonType),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<ScreenState>>,
    mut event_writer: EventWriter<PlaySoundEffectEvent>,
    mut exit_writer: EventWriter<AppExit>,
    theme: Res<UiTheme>,
) {
    for (interaction, mut bgcolor, button_type) in &mut interaction_q {
        match *interaction {
            Interaction::Pressed => {
                *bgcolor = theme.button_background_pressed.into();
                match button_type {
                    MenuButtonType::NewGame => next_state.set(ScreenState::Game),
                    MenuButtonType::Options => next_state.set(ScreenState::Option),
                    MenuButtonType::Credits => next_state.set(ScreenState::Credits),
                    MenuButtonType::Exit => {
                        exit_writer.write(AppExit::Success);
                    }
                }
                event_writer.write(PlaySoundEffectEvent(SfxSettings::new(
                    EffectName::UiConfirm,
                    Some(false),
                    None,
                )));
            }
            Interaction::Hovered => {
                event_writer.write(PlaySoundEffectEvent(SfxSettings::new(
                    EffectName::Click,
                    Some(false),
                    None,
                )));
                *bgcolor = theme.button_background_hover.into();
            }
            Interaction::None => {
                *bgcolor = theme.button_background_normal.into();
            }
        }
    }
}

//returns a node with the Button component designating interactivity
fn create_button_node(
    button_type: MenuButtonType,
    button_text: Text,
    commands: &mut Commands,
    theme: &Res<UiTheme>,
) -> Entity {
    let button_node = (
        Node {
            height: Val::Px(50.0),
            width: Val::Px(150.0),

            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(8.0)),
            flex_wrap: FlexWrap::NoWrap,
            ..default()
        },
        Button,
        BackgroundColor::from(Color::srgb(0.2, 0.1, 0.1)),
        BorderColor(Color::BLACK),
        button_type,
    );
    let button = commands.spawn(button_node).id();
    let button_text_node = commands
        .spawn((
            theme.font(),
            button_text,
            Node {
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
        ))
        .id();
    commands.entity(button).add_child(button_text_node);
    button
}

fn setup_ui_root(mut commands: Commands) {
    let parent_node = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        UiRoot,
        Pickable::IGNORE,
    );
    commands.spawn(parent_node);
}
fn setup_menu(
    mut commands: Commands,
    parent_node: Single<Entity, With<UiRoot>>,
    theme: Res<UiTheme>,
    asset_server: Res<AssetServer>,
) {
    info!("Setting up menu.");

    let title_card = commands
        .spawn((
            Node { ..default() },
            ImageNode {
                image: asset_server.load("image/banner.png"),
                ..default()
            },
        ))
        .id();
    let game_button = create_button_node(
        MenuButtonType::NewGame,
        Text::from("New Game"),
        &mut commands,
        &theme,
    );
    let options_button = create_button_node(
        MenuButtonType::Options,
        Text::from("Options"),
        &mut commands,
        &theme,
    );
    let credits_button = create_button_node(
        MenuButtonType::Credits,
        Text::from("Credits"),
        &mut commands,
        &theme,
    );

    #[cfg(not(target_arch = "wasm32"))]
    let exit_button = create_button_node(
        MenuButtonType::Exit,
        Text::from("Exit"),
        &mut commands,
        &theme,
    );

    #[cfg(target_arch = "wasm32")]
    let exit_button = commands.spawn_empty().id();

    //and then add the children (buttons)
    commands.entity(*parent_node).add_children(&[
        title_card,
        game_button,
        options_button,
        credits_button,
        exit_button,
    ]);
}

pub fn deconstruct_main_menu(mut commands: Commands, uiroot: Single<Entity, With<UiRoot>>) {
    info!("DECONSTRUCTING MAIN MENU");
    commands.entity(*uiroot).despawn_related::<Children>();
}
