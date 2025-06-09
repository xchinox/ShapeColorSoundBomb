use bevy::{audio::Volume, prelude::*};

use components::*;

use crate::screen::components::ScreenState;

pub mod components;
pub struct AudioServerPlugin;

impl Plugin for AudioServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioLibrary::default())
            .add_event::<PlaySoundEffectEvent>()
            .add_event::<MusicVolumeChangedEvent>()
            .add_event::<MasterVolumeChangedEvent>()
            .add_systems(Startup, setup_audio_server)
            .add_systems(OnExit(ScreenState::Splash), play_music)
            .insert_resource(AudioLibrary::default())
            .insert_resource(AudioSettings::default())
            .add_systems(Update, (play_sfx, clear_sfx))
            .add_systems(
                Update,
                (music_volume, master_volume).after(setup_audio_server),
            );
    }
}

pub fn setup_audio_server(mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.sfx_vol = 0.3;
    audio_settings.master_vol = 0.3;
    audio_settings.music_vol = 0.3;

    info!("Audio Server Plugin initalized");
}

fn play_music(
    audio_library: ResMut<AudioLibrary>,
    mut commands: Commands,
    audio_settings: Res<AudioSettings>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        AudioPlayer::new(
            asset_server.load(
                audio_library
                    .music
                    .get(&MusicTrackName::TrackOne)
                    .unwrap()
                    .clone(),
            ),
        ),
        MusicTrack,
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: Volume::Linear(audio_settings.music_vol),
            ..default()
        },
    ));
}

pub fn play_sfx(
    mut commands: Commands,
    mut event_reader: EventReader<PlaySoundEffectEvent>,
    audio_library: Res<AudioLibrary>,
    asettings: Res<AudioSettings>,
    asset_server: Res<AssetServer>,
) {
    for event in event_reader.read() {
        let sfx_settings = &event.0;

        let pitch = if sfx_settings.varied() {
            rand::random_range(0.75..1.25)
        } else {
            1.0
        };

        let playback_settings = PlaybackSettings {
            volume: bevy::audio::Volume::Linear(asettings.sfx_vol),
            speed: pitch,
            ..default()
        };

        let path = *audio_library.sfx.get(&event.0.get_name()).unwrap();
        let handle = asset_server.load(path);
        let audio_player = AudioPlayer::new(handle);
        commands.spawn((audio_player, playback_settings, SoundEffect));
    }
}

pub fn music_volume(
    mut event_reader: EventReader<MusicVolumeChangedEvent>,
    mut sink_q: Query<&mut AudioSink, With<MusicTrack>>,
) {
    if let Ok(mut sink) = sink_q.single_mut() {
        for event in event_reader.read() {
            if !event.0 {
                let newvol = sink.volume() - Volume::Linear(0.1);
                sink.set_volume(newvol);
            } else {
                let newvol = sink.volume() + Volume::Linear(0.1);
                sink.set_volume(newvol);
            }
        }
    }
}

pub fn master_volume(
    mut event_reader: EventReader<MasterVolumeChangedEvent>,
    mut sinks_q: Query<&mut AudioSink>,
    mut vol: ResMut<GlobalVolume>,
) {
    for event in event_reader.read() {
        if !event.0 {
            if vol.volume - Volume::Linear(0.1) >= Volume::Linear(0.0) {
                let dec_val = vol.volume - Volume::Linear(0.1);
                vol.volume = dec_val;
            }
            for mut sink in sinks_q.iter_mut() {
                let newvol = sink.volume() - Volume::Linear(0.1);
                sink.set_volume(newvol);
            }
        } else {
            let inc_val = vol.volume + Volume::Linear(0.1);
            vol.volume = inc_val;
            for mut sink in sinks_q.iter_mut() {
                let newvol = sink.volume() + Volume::Linear(0.1);
                sink.set_volume(newvol);
            }
        }
    }
}

pub fn clear_sfx(mut commands: Commands, sfx_q: Query<(Entity, &AudioSink), With<SoundEffect>>) {
    for (entity, sink) in sfx_q.iter() {
        if sink.empty() {
            commands.entity(entity).despawn();
        }
    }
}
