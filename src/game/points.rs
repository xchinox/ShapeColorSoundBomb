
use bevy::prelude::*;

use crate::{
    game::{
        bomb::{BombPiece, MatchBomb},
        cell_line::{CellLine, LineCompletedEvent},
        game_grid::GameGrid,
    },
    screen::components::ScreenState,
};

const BASE_SCORE: i32 = 10;

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::new())
            .add_systems(
                Update,
                on_line_complete.run_if(on_event::<LineCompletedEvent>),
            )
            .add_systems(OnEnter(ScreenState::Game), setup_score_display)
            .add_systems(
                Update,
                update_score_display.run_if(in_state(ScreenState::Game)),
            );
    }
}

//Components
#[derive(Event, Default)]
pub struct PointsScoredEvent(pub u32);

#[derive(Resource)]
pub struct Score {
    total: i32,
    perfects: i32,
    doubles: i32,
}

impl Score {
    fn new() -> Self {
        Score {
            total: 0,
            perfects: 0,
            doubles: 0,
        }
    }
}

//Because the name floating points was already taken by math dorks
#[derive(Component)]
pub struct ScrollingPoints(pub u32);

#[derive(Component)]
pub struct ScoreDisplay;
//Systems
//
//
//
fn setup_score_display(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::VMin(5.0),
            left: Val::Percent(50.0),
            width: Val::Percent(50.0),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
        Text::from(score.total.to_string()),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        ScoreDisplay,
        Transform::from_translation(Vec3::ZERO),
    ));
}

fn update_score_display(
    mut commands: Commands,
    text_q: Query<Entity, With<ScoreDisplay>>,
    bomb: Res<MatchBomb>,
) {
    if bomb.is_changed() {
        for text in text_q {
            commands
                .entity(text)
                .insert(Text::from(bomb.points_remaining().to_string()));
        }
    }
}

fn on_line_complete(
    cell_line: ResMut<CellLine>,
    grid: Res<GameGrid>,
    mut score: ResMut<Score>,
    mut bomb: ResMut<MatchBomb>,
    bomb_piece: Single<&BombPiece>,
) {
    let mut total = 0;
    for (i, position) in cell_line.visited.iter().enumerate() {
        //Can't match the last element
        total = BASE_SCORE * cell_line.visited.len() as i32 * ((score.perfects + 1) * 2)
            + (score.doubles + 1);
        score.total += total;
    }

    if cell_line.visited.contains(&bomb_piece.position()) {
        bomb.sub(total.try_into().unwrap());
    }
    bomb.decrement();
}
