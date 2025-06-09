use bevy::prelude::*;

use super::{components::ScreenState, menu::UiRoot};
use crate::ui_theme::*;
pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ScreenState::Credits), setup_credits)
            .add_systems(
                Update,
                return_to_menu.run_if(in_state(ScreenState::Credits)),
            )
            .add_systems(OnExit(ScreenState::Credits), deconstruct_credits_menu);
    }
}

pub fn setup_credits(
    mut commands: Commands,
    uiroot: Single<Entity, With<super::menu::UiRoot>>,
    theme: Res<UiTheme>,
) {
    let title_text = commands
        .spawn((Node { ..default() }, Text::from("Credits:"), theme.font()))
        .id();

    let credits_text = commands
        .spawn((
            Node { ..default() },
            Text::from(" Made by xchino."),
            theme.font(),
        ))
        .id();

    let bevy_text = commands
        .spawn((
            Node { ..default() },
            Text::from(" With Bevy Engine."),
            theme.font(),
        ))
        .id();
    let flock_text = commands
        .spawn((
            Node { ..default() },
            Text::from("And various code lifted from community template by TheBevyFlock."),
            theme.font(),
        ))
        .id();
    let font_text = commands
        .spawn((
            Node { ..default() },
            Text::from(" Using OpenDyselxic font."),
            theme.font(),
        ))
        .id();
    let others_text = commands
        .spawn((
            Node { ..default() },
            Text::from(" And countless other contributions from the Bevy community and ecosystem."),
            theme.font(),
        ))
        .id();

    commands.entity(*uiroot).add_children(&[
        title_text,
        credits_text,
        bevy_text,
        flock_text,
        font_text,
        others_text,
    ]);
}

pub fn return_to_menu(
    mut next_state: ResMut<NextState<ScreenState>>,
    input: ResMut<ButtonInput<KeyCode>>,
) {
    if input.just_released(KeyCode::Escape) {
        next_state.set(ScreenState::Menu);
    }
}

pub fn deconstruct_credits_menu(mut commands: Commands, uiroot: Single<Entity, With<UiRoot>>) {
    info!("Deconstructing Credits menu");
    commands.entity(*uiroot).despawn_related::<Children>();
}
