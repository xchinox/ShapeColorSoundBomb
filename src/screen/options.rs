use bevy::prelude::*;

use crate::audio_server::components::{
    AudioSettings, EffectName, MasterVolumeChangedEvent, MusicVolumeChangedEvent,
    PlaySoundEffectEvent, SfxSettings,
};

use super::{components::ScreenState, menu::UiRoot};
use crate::ui_theme::*;

#[derive(Component)]
pub enum OptionButtonType {
    MasterVolUp,
    MasterVolDown,
    SfxVolUp,
    SfxVolDown,
    MusicVolUp,
    MusicVolDown,
}
pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ScreenState::Option), setup_options)
            .add_systems(
                Update,
                option_buttons_system.run_if(in_state(ScreenState::Option)),
            )
            .add_systems(Update, return_to_menu.run_if(in_state(ScreenState::Option)))
            .add_systems(OnExit(ScreenState::Option), deconstruct_options_menu);
    }
}

pub fn return_to_menu(
    input: ResMut<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<ScreenState>>,
) {
    if input.just_released(KeyCode::Escape) {
        next_state.set(ScreenState::Menu);
    }
}

//This returns the entity id of an "incremental" button
pub fn create_incremental_button(
    commands: &mut Commands,
    button_type: OptionButtonType,
    text: String,
    theme: &Res<UiTheme>,
) -> Entity {
    let button = commands
        .spawn((
            Node {
                // c,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            Button,
            button_type,
            BackgroundColor(theme.button_background_normal),
            BorderColor(theme.button_border_normal),
        ))
        .id();

    let text_node = commands
        .spawn((
            Node {
                margin: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            Text::from(text),
            theme.font(),
        ))
        .id();

    commands.entity(button).add_child(text_node);
    button
}

pub fn setup_options(
    uiroot: Single<Entity, With<UiRoot>>,
    mut commands: Commands,
    theme: Res<UiTheme>,
) {
    info!("Constructing options menu");

    let master_row_container = commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            margin: UiRect::all(Val::Px(5.0)),
            align_items: AlignItems::Center,
            ..default()
        })
        .id();

    let effect_row_container = commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        })
        .id();

    let music_row_container = commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        })
        .id();

    let volume_label = commands
        .spawn((
            Node {
                ..Default::default()
            },
            Text::from("Volume"),
            theme.font(),
        ))
        .id();

    //Create the button entities
    let master_volup_button = create_incremental_button(
        &mut commands,
        OptionButtonType::MasterVolUp,
        String::from("+"),
        &theme,
    );
    let master_voldown_button = create_incremental_button(
        &mut commands,
        OptionButtonType::MasterVolDown,
        String::from("-"),
        &theme,
    );

    let sfx_volup_button = create_incremental_button(
        &mut commands,
        OptionButtonType::SfxVolUp,
        String::from("+"),
        &theme,
    );

    let sfx_voldown_button = create_incremental_button(
        &mut commands,
        OptionButtonType::SfxVolDown,
        String::from("-"),
        &theme,
    );

    let music_volup_button = create_incremental_button(
        &mut commands,
        OptionButtonType::MusicVolUp,
        String::from("+"),
        &theme,
    );
    let music_voldown_button = create_incremental_button(
        &mut commands,
        OptionButtonType::MusicVolDown,
        String::from("-"),
        &theme,
    );

    //Create the label entities
    let master_vol_text = commands
        .spawn((
            Node {
                margin: UiRect::all(Val::Px(5.0)),
                width: Val::Px(175.0),
                ..default()
            },
            Text::from("Main"),
            theme.font(),
            BackgroundColor(Color::BLACK),
        ))
        .id();

    let effect_vol_text = commands
        .spawn((
            Node {
                margin: UiRect::all(Val::Px(5.0)),
                width: Val::Px(175.0),
                ..default()
            },
            Text::from("Effects"),
            theme.font(),
        ))
        .id();

    let music_vol_text = commands
        .spawn((
            Node {
                margin: UiRect::all(Val::Px(5.0)),
                width: Val::Px(175.0),
                ..default()
            },
            Text::from("Music"),
            theme.font(),
        ))
        .id();

    //Add children to parent containers
    commands.entity(master_row_container).add_children(&[
        sfx_voldown_button,
        effect_vol_text,
        sfx_volup_button,
    ]);

    commands.entity(effect_row_container).add_children(&[
        master_voldown_button,
        master_vol_text,
        master_volup_button,
    ]);

    commands.entity(music_row_container).add_children(&[
        music_voldown_button,
        music_vol_text,
        music_volup_button,
    ]);

    //Add containers to the uiroot
    commands.entity(*uiroot).add_children(&[
        volume_label,
        master_row_container,
        music_row_container,
        effect_row_container,
    ]);
    info!("Setting up options menu");
}

//#TODO - just send events and handle volume in the audio server
pub fn option_buttons_system(
    mut interaction_q: Query<
        (&mut Interaction, &mut BackgroundColor, &OptionButtonType),
        (Changed<Interaction>, With<Button>),
    >,
    mut audio_settings: ResMut<AudioSettings>,
    mut sfx_event_writer: EventWriter<PlaySoundEffectEvent>,
    mut music_event_writer: EventWriter<MusicVolumeChangedEvent>,
    mut master_event_writer: EventWriter<MasterVolumeChangedEvent>,
    theme: Res<UiTheme>,
) {
    for (interaction, mut bgcolor, buttontype) in &mut interaction_q {
        match *interaction {
            Interaction::None => *bgcolor = BackgroundColor(theme.button_background_normal),
            Interaction::Hovered => *bgcolor = BackgroundColor(theme.button_background_hover),
            Interaction::Pressed => {
                *bgcolor = BackgroundColor(theme.button_background_pressed);
                match buttontype {
                    OptionButtonType::MusicVolUp => {
                        music_event_writer.write(MusicVolumeChangedEvent(true));
                    }
                    OptionButtonType::MusicVolDown => {
                        music_event_writer.write(MusicVolumeChangedEvent(false));
                    }

                    OptionButtonType::MasterVolDown => {
                        master_event_writer.write(MasterVolumeChangedEvent(false));
                        sfx_event_writer.write(PlaySoundEffectEvent(SfxSettings::new(
                            EffectName::UiConfirm,
                            Some(false),
                            None,
                        )));
                    }
                    OptionButtonType::MasterVolUp => {
                        master_event_writer.write(MasterVolumeChangedEvent(true));
                        sfx_event_writer.write(PlaySoundEffectEvent(SfxSettings::new(
                            EffectName::UiConfirm,
                            Some(false),
                            None,
                        )));
                    }
                    OptionButtonType::SfxVolDown => {
                        if audio_settings.sfx_vol - 0.1 >= 0.0 {
                            //Don't allow the volume to go into negatives
                            audio_settings.sfx_vol -= 0.1;
                        }
                        sfx_event_writer.write(PlaySoundEffectEvent(SfxSettings::new(
                            EffectName::UiConfirm,
                            Some(false),
                            None,
                        )));
                    }
                    OptionButtonType::SfxVolUp => {
                        audio_settings.sfx_vol += 0.1;
                        sfx_event_writer.write(PlaySoundEffectEvent(SfxSettings::new(
                            EffectName::UiConfirm,
                            Some(false),
                            None,
                        )));
                    }
                }
            }
        }
    }
}

pub fn deconstruct_options_menu(mut commands: Commands, uiroot: Single<Entity, With<UiRoot>>) {
    info!("Deconstruct Options Menu");
    commands.entity(*uiroot).despawn_related::<Children>();
}
