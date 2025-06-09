
use bevy::prelude::*;

use crate::{
    audio_server::components::{EffectName, PlaySoundEffectEvent, SfxSettings},
    game::{
        GameState,
        bomb::BombPiece,
        cell_line::{CellLine, CellVisitedEvent},
        game_board::BoardPiece,
        game_grid::*,
    },
};

pub struct InputHandlerPlugin;

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input);
    }
}

pub fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut cell_line: ResMut<CellLine>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::FreePick);
        cell_line.visited.clear();
    }
}

pub fn on_over(
    trigger: Trigger<Pointer<Over>>,
    piece_q: Query<(Entity, &BoardPiece)>,
    bomb: Single<Entity, With<BombPiece>>,
    mut ew_sfx_player: EventWriter<PlaySoundEffectEvent>,
) {
    if trigger.target() == *bomb {
        ew_sfx_player.write(PlaySoundEffectEvent(SfxSettings::new(
            EffectName::Fuse,
            Some(true),
            None,
        )));
    }
    for (entity, piece) in piece_q.iter() {
        if trigger.target() == entity {
            let note = match piece.game_piece.sound {
                PieceSound::A => EffectName::NoteA,
                PieceSound::B => EffectName::NoteB,
                PieceSound::C => EffectName::NoteC,
                PieceSound::D => EffectName::NoteD,
                PieceSound::E => EffectName::NoteE,
                PieceSound::F => EffectName::NoteF,
                PieceSound::G => EffectName::NoteG,
            };

            ew_sfx_player.write(PlaySoundEffectEvent(SfxSettings::new(note, None, None)));
        }
    }
}
pub fn on_click(
    trigger: Trigger<Pointer<Pressed>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ew_init_cell_line: EventWriter<CellVisitedEvent>,
    piece_q: Query<(Entity, &BoardPiece)>,
    grid: Res<GameGrid>,
) {
    if state.get() == &GameState::FreePick {
        next_state.set(GameState::PickNext);
        for (entity, piece) in piece_q.iter() {
            if trigger.target() == entity {
                ew_init_cell_line.write(CellVisitedEvent(grid.get_position(&piece.game_piece)));
            }
        }
    }

    if state.get() == &GameState::PickNext {
        for (entity, piece) in piece_q.iter() {
            if trigger.target() == entity {
                ew_init_cell_line.write(CellVisitedEvent(grid.get_position(&piece.game_piece)));
            }
        }
    }
}

pub fn on_bomb_click(
    _trigger: Trigger<Pointer<Pressed>>,
    mut cell_line: ResMut<CellLine>,
    bomb_q: Query<&BombPiece>,
) {
    for bomb in bomb_q {
        let nv = Vec2::new(bomb.0.0 as f32, bomb.0.1 as f32);
        cell_line.visited.push(nv);
    }
}
