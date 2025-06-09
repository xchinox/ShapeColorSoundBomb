use std::collections::HashMap;
use std::f32::consts::TAU;
use std::time::Duration;

use crate::audio_server::components::{EffectName, PlaySoundEffectEvent, SfxSettings};
use crate::game::bomb::BombPiece;
use crate::game::cell_line::CellLine;
use crate::game::input_handler::{on_bomb_click, on_over};
use crate::{game::game_grid::*, screen::components::ScreenState};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::scene::SceneInstanceReady;
use rand::Rng;

pub struct GameBoardPlugin;

impl Plugin for GameBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DespawnQueue(vec![]))
            .insert_resource(DespawnTimer(Timer::new(
                Duration::from_secs_f32(0.25),
                TimerMode::Repeating,
            )))
            .add_event::<UpdateBoardEvent>()
            .add_systems(
                OnEnter(ScreenState::Game),
                (setup_board_system, setup_despawn_timer),
            )
            .add_systems(Update, update_board.run_if(on_event::<UpdateBoardEvent>))
            .add_systems(Update, on_pop_cell.run_if(on_event::<PopCellEvent>))
            .add_systems(Startup, (load_model_map, load_material_map))
            .add_systems(
                Update,
                (rotate_pieces, pop_cell).run_if(in_state(ScreenState::Game)),
            )
            .add_observer(apply_material);
    }
}

#[derive(Event, Default)]
pub struct UpdateBoardEvent;

#[derive(Resource)]
pub struct ModelMap(HashMap<PieceShape, Handle<Scene>>);

#[derive(Resource)]
pub struct MaterialMap(HashMap<PieceColor, Handle<StandardMaterial>>);

//Marker for the parent to which pieces are added as children
#[derive(Component)]
pub struct BoardContainer;

//Board Piece
#[derive(Component, Debug)]
pub struct BoardPiece {
    pub game_piece: GamePiece,
}

impl BoardPiece {
    fn new(game_piece: GamePiece) -> Self {
        BoardPiece { game_piece }
    }
}

#[derive(Resource, Debug)]
pub struct DespawnQueue(pub Vec<Entity>);

#[derive(Resource, Debug)]
pub struct DespawnTimer(pub Timer);

//Systems
//

fn setup_despawn_timer(mut despawn_timer: ResMut<DespawnTimer>) {
    let timer = Timer::new(
        Duration::try_from_secs_f32(0.15).unwrap(),
        TimerMode::Repeating,
    );
    despawn_timer.0 = timer;
}
fn load_model_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map: HashMap<PieceShape, Handle<Scene>> = HashMap::from([
        (
            PieceShape::Circle,
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/circle.glb")),
        ),
        (
            PieceShape::Square,
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/cuboid.glb")),
        ),
        (
            PieceShape::Triangle,
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/pyramid.glb")),
        ),
        (
            PieceShape::X,
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/x.glb")),
        ),
        (
            PieceShape::Plus,
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/plus.glb")),
        ),
        (
            PieceShape::Diamond,
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/diamond.glb")),
        ),
        (
            PieceShape::Bomb,
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/bomb.glb")),
        ),
    ]);

    commands.insert_resource(ModelMap(map));
}

fn load_material_map(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mat_colors = [
        PieceColor::Pink,
        PieceColor::Green,
        PieceColor::Blue,
        PieceColor::Yellow,
        PieceColor::Orange,
        PieceColor::Purple,
        PieceColor::Cyan,
        PieceColor::Red,
    ];

    let mat_map: HashMap<PieceColor, Handle<StandardMaterial>> = mat_colors
        .into_iter()
        .map(|color| (color, materials.add(color.to_color())))
        .collect();

    commands.insert_resource(MaterialMap(mat_map));
}

fn setup_board_system(
    mut commands: Commands,
    gg: Res<GameGrid>,
    model_map: ResMut<ModelMap>,
    asset_server: Res<AssetServer>,
) {
    let container = commands
        .spawn((BoardContainer, Transform::from_translation(Vec3::ZERO)))
        .id();

    let mut bomb_pos: (usize, usize) = (0, 0);
    let mut rng = rand::rng();
    bomb_pos.0 = rng.random_range(0..gg.cells.rows());
    bomb_pos.1 = rng.random_range(0..gg.cells.cols());

    for (i, cell) in gg.cells.indexed_iter() {
        if i == bomb_pos {
            let model = model_map.0.get(&PieceShape::Bomb).unwrap();

            let child = commands
                .spawn((
                    BombPiece(bomb_pos),
                    Transform::from_translation(Vec3::new(i.0 as f32 * 2.0, i.1 as f32 * 2.0, 0.0)),
                    SceneRoot(model.clone()),
                ))
                .observe(on_bomb_click)
                .observe(on_over)
                .id();
            commands.entity(container).add_child(child);
        } else if let Some(piece) = cell {
            let model = match piece.shape {
                PieceShape::Circle => model_map.0.get(&PieceShape::Circle).unwrap(),
                PieceShape::Square => model_map.0.get(&PieceShape::Square).unwrap(),
                PieceShape::Triangle => model_map.0.get(&PieceShape::Triangle).unwrap(),
                PieceShape::X => model_map.0.get(&PieceShape::X).unwrap(),
                PieceShape::Plus => model_map.0.get(&PieceShape::Plus).unwrap(),
                PieceShape::Diamond => model_map.0.get(&PieceShape::Diamond).unwrap(),
                PieceShape::Bomb => model_map.0.get(&PieceShape::Bomb).unwrap(),
            };

            let child = commands
                .spawn((
                    BoardPiece::new(*piece),
                    Transform::from_translation(Vec3::new(i.0 as f32 * 2.0, i.1 as f32 * 2.0, 0.0)),
                    SceneRoot(model.clone()),
                    piece.color,
                    RenderLayers::layer(0),
                ))
                .observe(super::input_handler::on_click)
                .observe(super::input_handler::on_over)
                .id();

            commands.entity(container).add_child(child);
        }
    }
}

fn update_board(
    mut commands: Commands,
    container: Single<Entity, With<BoardContainer>>,
    model_map: ResMut<ModelMap>,
    gg: Res<GameGrid>,
    mut cell_line: ResMut<CellLine>,
) {
    cell_line.visited.clear();
    commands.entity(*container).despawn_related::<Children>();
    let mut bomb_pos: (usize, usize) = (0, 0);
    let mut rng = rand::rng();
    bomb_pos.0 = rng.random_range(0..gg.cells.rows());
    bomb_pos.1 = rng.random_range(0..gg.cells.cols());

    for (i, cell) in gg.cells.indexed_iter() {
        if i == bomb_pos {
            let model = model_map.0.get(&PieceShape::Bomb).unwrap();
            let child = commands
                .spawn((
                    BombPiece(bomb_pos),
                    Transform::from_translation(Vec3::new(i.0 as f32 * 2.0, i.1 as f32 * 2.0, 0.0)),
                    SceneRoot(model.clone()),
                ))
                .observe(on_bomb_click)
                .observe(on_over)
                .id();

            commands.entity(*container).add_child(child);
        } else if let Some(piece) = cell {
            let model = match piece.shape {
                PieceShape::Circle => model_map.0.get(&PieceShape::Circle).unwrap(),
                PieceShape::Square => model_map.0.get(&PieceShape::Square).unwrap(),
                PieceShape::Triangle => model_map.0.get(&PieceShape::Triangle).unwrap(),
                PieceShape::X => model_map.0.get(&PieceShape::X).unwrap(),
                PieceShape::Plus => model_map.0.get(&PieceShape::Plus).unwrap(),
                PieceShape::Diamond => model_map.0.get(&PieceShape::Diamond).unwrap(),
                PieceShape::Bomb => model_map.0.get(&PieceShape::Bomb).unwrap(),
            };

            let child = commands
                .spawn((
                    BoardPiece::new(*piece),
                    Transform::from_translation(Vec3::new(i.0 as f32 * 2.0, i.1 as f32 * 2.0, 0.0)),
                    SceneRoot(model.clone()),
                    piece.color,
                ))
                .observe(super::input_handler::on_over)
                .observe(super::input_handler::on_click)
                .id();

            commands.entity(*container).add_child(child);
        }
    }
}
fn apply_material(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
    material_map: Res<MaterialMap>,
    mut asset_materials: ResMut<Assets<StandardMaterial>>,
    piece: Query<&BoardPiece>,
    asset_server: Res<AssetServer>,
) {
    for descendents in children.iter_descendants(trigger.target()) {
        let Ok(piece) = piece.get(trigger.target()) else {
            return;
        };
        if let Some(material) = mesh_materials
            .get(descendents)
            .ok()
            .and_then(|id| asset_materials.get_mut(id.id()))
        {
            let mut new_material = material.clone();
            new_material.base_color = piece.game_piece.color.to_color();
            new_material.base_color_texture = Some(asset_server.load("image/purple_concrete.png"));
            new_material.emissive = piece.game_piece.color.to_color().to_linear();

            commands
                .entity(descendents)
                .insert(MeshMaterial3d(asset_materials.add(new_material)));
        }
    }
}

fn rotate_pieces(mut pieces_q: Query<&mut Transform, With<BoardPiece>>, time: Res<Time>) {
    for mut piece in pieces_q.iter_mut() {
        piece.rotate_y(0.03 * TAU * time.delta_secs().sin());
        //        piece.rotate_z(0.03 * TAU * time.delta_secs().sin());
        piece.rotate_x(0.03 * TAU * time.delta_secs().sin());
    }
}

fn on_pop_cell(
    mut er_pop_cell: EventReader<PopCellEvent>,
    piece_q: Query<(Entity, &BoardPiece)>,
    grid: Res<GameGrid>,
    mut despawn_queue: ResMut<DespawnQueue>,
) {
    for event in er_pop_cell.read() {
        let target = event.0;
        for (entity, piece) in piece_q {
            if grid.get_position(&piece.game_piece) == target {
                despawn_queue.0.push(entity);
            }
        }
    }
}

fn pop_cell(
    mut commands: Commands,
    mut despawn_queue: ResMut<DespawnQueue>,
    mut timer: ResMut<DespawnTimer>,
    time: Res<Time>,
    mut ew_sfx_player: EventWriter<PlaySoundEffectEvent>,
    mut ew_update_board: EventWriter<UpdateBoardEvent>,
    mut grid: ResMut<GameGrid>,
) {
    timer.0.tick(time.delta());

    if despawn_queue.0.is_empty() {
        return;
    }

    if timer.0.just_finished() {
        commands.entity(despawn_queue.0[0]).despawn();

        ew_sfx_player.write(PlaySoundEffectEvent(SfxSettings::new(
            EffectName::BubblePop,
            Some(true),
            None,
        )));
        despawn_queue.0.remove(0);
        if despawn_queue.0.is_empty() {
            let new_grid = GameGrid::new();
            *grid = new_grid;
            ew_update_board.write_default();
        }
    }
}
