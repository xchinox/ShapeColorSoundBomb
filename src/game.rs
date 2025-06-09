use crate::{
    audio_server::components::{EffectName, PlaySoundEffectEvent, SfxSettings},
    game::cell_line::CellLinePlugin,
    screen::{components::*, menu::UiRoot},
};
use bevy::prelude::*;
pub struct GamePlugin;

pub mod game_grid;
use game_grid::*;

pub mod game_board;
use game_board::*;

pub mod input_handler;
use input_handler::*;

pub mod cell_line;

pub mod points;
use points::*;

pub mod bomb;
use bomb::*;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::FreePick)
            .insert_resource(ClearColor(Color::BLACK))
            .add_plugins(GameGridPlugin)
            .add_plugins((
                GameBoardPlugin,
                InputHandlerPlugin,
                CellLinePlugin,
                PointsPlugin,
                BombPlugin,
            ))
            .add_systems(OnEnter(ScreenState::Game), setup_game)
            .add_systems(OnEnter(GameState::GameOver), game_over)
            .add_systems(Update, game_won.run_if(in_state(ScreenState::Game)));
    }
}

#[derive(Component)]
pub struct HudDisplay;

//Marker for the gameover model
#[derive(Component)]
pub struct GameOverText;
#[derive(States, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameState {
    FreePick,
    PickNext,
    GameOver,
}

pub fn setup_game(
    mut commands: Commands,
    mut ew_initgrid: EventWriter<InitializeGridEvent>,
    uiroot: Single<Entity, With<UiRoot>>,
) {
    info!("Initializing Game");

    commands.entity(*uiroot).despawn();
    ew_initgrid.write_default();
}

pub fn game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ew_sfx_player: EventWriter<PlaySoundEffectEvent>,
) {
    let model = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/game_over.glb"));
    commands
        .spawn((
            SceneRoot(model.clone()),
            Transform::from_translation(Vec3::new(7.7, 7.7, 5.0)),
            GameOverText,
            Pickable {
                should_block_lower: true,
                ..default()
            },
        ))
        .observe(on_retry);
    ew_sfx_player.write(PlaySoundEffectEvent(SfxSettings::new(
        EffectName::Detonation,
        None,
        None,
    )));
}

pub fn game_won(
    commands: Commands,
    mut bomb: ResMut<MatchBomb>,
    mut ew_sfx_player: EventWriter<PlaySoundEffectEvent>,
    mut ew_bomb_defused: EventWriter<BombDefusedEvent>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    if bomb.points_remaining() == 0 {
        bomb.rearm();
        ew_sfx_player.write(PlaySoundEffectEvent(SfxSettings::new(
            EffectName::FanFare,
            None,
            None,
        )));

        ew_sfx_player.write(PlaySoundEffectEvent(SfxSettings::new(
            EffectName::Fuse,
            None,
            None,
        )));
        ew_bomb_defused.write_default();
    }
}

fn on_retry(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut bomb: ResMut<MatchBomb>,
    game_over: Single<Entity, With<GameOverText>>,
) {
    bomb.reset();
    commands.entity(*game_over).despawn();
}
