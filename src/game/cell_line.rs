use bevy::{color::palettes::tailwind::RED_950, prelude::*};

use crate::{
    audio_server::components::{EffectName, PlaySoundEffectEvent, SfxSettings},
    game::{
        bomb::BombPiece,
        game_grid::{GameGrid, GamePiece, PopCellEvent},
    },
    screen::components::ScreenState,
};

pub struct CellLinePlugin;

impl Plugin for CellLinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CellVisitedEvent>()
            .add_event::<LineCompletedEvent>()
            .insert_resource(CellLine::new())
            .add_systems(
                Update,
                (update_cell_visitation, draw_line).run_if(in_state(ScreenState::Game)),
            )
            .add_systems(
                Update,
                on_line_complete.run_if(on_event::<LineCompletedEvent>),
            );
    }
}

#[derive(Event)]
pub struct CellVisitedEvent(pub Vec2);

#[derive(Event)]
pub struct LineCompletedEvent;

#[derive(Resource, Clone, Debug)]
pub struct CellLine {
    pub visited: Vec<Vec2>,
}

impl CellLine {
    fn new() -> Self {
        CellLine { visited: vec![] }
    }

    fn visit(&mut self, pos: Vec2) {
        self.visited.push(pos);
    }

    pub fn validate(&self, target: &GamePiece, source: &GamePiece, grid: &GameGrid) -> bool {
        let mut retval = true;
        // Do they share a property?
        if target.color != source.color
            && target.shape != source.shape
            && target.sound != source.sound
        {
            retval = false
        }

        //Are they neighbors?
        if !CellLine::is_neighbor(grid.get_position(target), grid.get_position(source)) {
            retval = false
        }

        //Cant go back
        if self.visited.contains(&grid.get_position(target)) {
            retval = false
        }
        let mut segments = self.visited.clone();
        segments.push(grid.get_position(target));
        if CellLine::has_self_intersections(segments.as_slice()) {
            retval = false
        }

        retval
    }

    //Determine if target intersects path
    fn has_self_intersections(points: &[Vec2]) -> bool {
        fn orientation(p: Vec2, q: Vec2, r: Vec2) -> i32 {
            let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
            if val.abs() < f32::EPSILON {
                0 // colinear
            } else if val > 0.0 {
                1 // clockwise
            } else {
                2 // counterclockwise
            }
        }

        fn on_segment(p: Vec2, q: Vec2, r: Vec2) -> bool {
            q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
        }

        fn segments_intersect(p1: Vec2, q1: Vec2, p2: Vec2, q2: Vec2) -> bool {
            let o1 = orientation(p1, q1, p2);
            let o2 = orientation(p1, q1, q2);
            let o3 = orientation(p2, q2, p1);
            let o4 = orientation(p2, q2, q1);

            if o1 != o2 && o3 != o4 {
                return true;
            }

            if o1 == 0 && on_segment(p1, p2, q1) {
                return true;
            }
            if o2 == 0 && on_segment(p1, q2, q1) {
                return true;
            }
            if o3 == 0 && on_segment(p2, p1, q2) {
                return true;
            }
            if o4 == 0 && on_segment(p2, q1, q2) {
                return true;
            }

            false
        }

        for i in 0..points.len().saturating_sub(1) {
            for j in i + 2..points.len().saturating_sub(1) {
                if i == 0 && j == points.len() - 2 {
                    continue; // skip closing segment if polyline is closed
                }
                let p1 = points[i];
                let q1 = points[i + 1];
                let p2 = points[j];
                let q2 = points[j + 1];
                if segments_intersect(p1, q1, p2, q2) {
                    return true;
                }
            }
        }
        false
    }

    fn is_neighbor(target: Vec2, source: Vec2) -> bool {
        let dx = (source.x - target.x).abs();
        let dy = (source.y - target.y).abs();
        (dx <= 1.0 && dy <= 1.0) && (dx > 0.0 || dy > 0.0)
    }
}

///Check that target cell is valid
fn update_cell_visitation(
    mut er_visited: EventReader<CellVisitedEvent>,
    mut cell_line: ResMut<CellLine>,
    game_grid: Res<GameGrid>,
    mut ew_line_complete: EventWriter<LineCompletedEvent>,
    mut ew_sfx_player: EventWriter<PlaySoundEffectEvent>,
    bomb: Single<&BombPiece>,
) {
    for event in er_visited.read() {
        if !cell_line.visited.is_empty() {
            if let Some(target) = game_grid.get_piece(event.0)
                && let Some(source) = game_grid.get_piece(*cell_line.visited.last().unwrap())
            {
                if source.id() == target.id() {
                    return;
                }
                let diff_vec = (game_grid.get_position(target) - bomb.position()).abs();

                if cell_line.validate(target, source, &game_grid)
                    || diff_vec.x <= 1.0 && diff_vec.y <= 1.0
                {
                    if !has_out(
                        &game_grid,
                        game_grid.get_position(target),
                        cell_line.clone(),
                        &bomb,
                    ) {
                        ew_line_complete.write(LineCompletedEvent);
                    }
                    ew_sfx_player.write(PlaySoundEffectEvent(SfxSettings::new(
                        EffectName::ValidSelection,
                        Some(true),
                        None,
                    )));
                    cell_line.visited.push(event.0);
                } else {
                    ew_line_complete.write(LineCompletedEvent);
                }
            }

        //Initialize the starting point
        } else {
            cell_line.visited.push(event.0);
            ew_sfx_player.write(PlaySoundEffectEvent(SfxSettings::new(
                EffectName::ValidSelection,
                Some(true),
                None,
            )));
        }
    }
}

fn has_out(
    game_grid: &GameGrid,
    target: Vec2,
    cell_line: CellLine,
    bomb: &Single<&BombPiece>,
) -> bool {
    let diff_vec = (target - bomb.position()).abs();
    if diff_vec.x <= 1.0 && diff_vec.y <= 1.0 {
        return true;
    }
    if !game_grid.check_neighbors(target, cell_line, game_grid) {
        return false;
    }
    true
}

///Draws the line segments for visited cells
fn draw_line(mut gizmos: Gizmos, cell_line: ResMut<CellLine>) {
    let mut prev_point: Vec2 = Vec2::NEG_ONE;
    for point in cell_line.visited.iter() {
        if prev_point != Vec2::NEG_ONE {
            gizmos.line_gradient(
                prev_point.extend(0.0) * 2.0,
                point.extend(0.0) * 2.0,
                Color::srgba(5.0, 0.0, 5.0, 1.0),
                Color::from(RED_950),
            );
            prev_point = *point;
        } else {
            prev_point = *point;
        }
    }
}

fn on_line_complete(
    mut cell_line: ResMut<CellLine>,
    mut game_grid: ResMut<GameGrid>,
    mut ew_pop_cell: EventWriter<PopCellEvent>,
    mut ew_sfx_player: EventWriter<PlaySoundEffectEvent>,
) {
    if cell_line.visited.len() <= 3 {
        cell_line.visited.clear();
        ew_sfx_player.write(PlaySoundEffectEvent(SfxSettings::new(
            EffectName::Negative,
            Some(true),
            None,
        )));
        return;
    }
    for cell in cell_line.visited.iter() {
        game_grid.pop_cell(*cell);
        ew_pop_cell.write(PopCellEvent(*cell));
    }

    let mut new_grid_vec = vec![];
    for i in 0..game_grid.cells.cols() {
        let mut new_col = GameGrid::collapse_column(game_grid.cells.clone(), i);
        new_grid_vec.append(&mut new_col);
    }
}
