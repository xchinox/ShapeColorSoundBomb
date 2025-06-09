use bevy::{platform::collections::HashMap, prelude::*};
// AudioServer Components
//
//
//

#[derive(Resource, Default)]
pub struct AudioSettings {
    pub master_vol: f32,
    pub music_vol: f32,
    pub sfx_vol: f32,
}

//To add an SFX audio source create a new element
//then define its path in the AudioLibrary Default impl
#[derive(Component, Eq, PartialEq, Hash, Debug, Clone)]
pub enum EffectName {
    Click,
    UiConfirm,
    ValidSelection,
    NoteA,
    NoteB,
    NoteC,
    NoteD,
    NoteE,
    NoteF,
    NoteG,
    BubblePop,
    Fuse,
    FanFare,
    Detonation,
    Negative,
}

#[derive(Resource)]
pub struct AudioLibrary {
    pub sfx: HashMap<EffectName, &'static str>,
    pub music: HashMap<MusicTrackName, &'static str>,
}

impl Default for AudioLibrary {
    fn default() -> Self {
        AudioLibrary {
            sfx: HashMap::from([
                (EffectName::Click, "sfx/click.ogg"),
                (EffectName::UiConfirm, "sfx/sine_boop.ogg"),
                (EffectName::ValidSelection, "sfx/select.ogg"),
                (EffectName::NoteA, "sfx/organ_a.ogg"),
                (EffectName::NoteB, "sfx/organ_b.ogg"),
                (EffectName::NoteC, "sfx/organ_c.ogg"),
                (EffectName::NoteD, "sfx/organ_d.ogg"),
                (EffectName::NoteE, "sfx/organ_e.ogg"),
                (EffectName::NoteF, "sfx/organ_f.ogg"),
                (EffectName::NoteG, "sfx/organ_g.ogg"),
                (EffectName::BubblePop, "sfx/bubble_pop.ogg"),
                (EffectName::Fuse, "sfx/fuse_quick.ogg"),
                (EffectName::FanFare, "sfx/ba_bum_fanfare.ogg"),
                (EffectName::Detonation, "sfx/cinematic_explosion.ogg"),
                (EffectName::Negative, "sfx/negative.ogg"),
            ]),

            music: HashMap::from([(MusicTrackName::TrackOne, "music/static.ogg")]),
        }
    }
}
pub struct SfxSettings {
    name: EffectName,
    varied: Option<bool>,
    position: Vec3,
}

impl SfxSettings {
    pub fn new(name: EffectName, varied: Option<bool>, position: Option<Vec3>) -> Self {
        let fx_position = if let Some(this_position) = position {
            this_position
        } else {
            Vec3::ZERO
        };

        SfxSettings {
            name,
            varied,
            position: fx_position,
        }
    }

    pub fn get_name(&self) -> EffectName {
        self.name.clone()
    }

    pub fn varied(&self) -> bool {
        if self.varied.is_some() {
            self.varied.unwrap()
        } else {
            false
        }
    }
}

#[derive(Component, Eq, PartialEq, Hash, Debug)]
pub enum MusicTrackName {
    TrackOne,
}

//When this event is fired the sound associated with the EffectName enum is played.
#[derive(Event)]
pub struct PlaySoundEffectEvent(pub SfxSettings);

#[derive(Event)]
pub struct MusicVolumeChangedEvent(pub bool);

#[derive(Event)]
pub struct MasterVolumeChangedEvent(pub bool);

//Marker component for music track
#[derive(Component)]
pub struct MusicTrack;

#[derive(Component)]
pub struct SoundEffect;
