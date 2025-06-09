use std::{f32::consts::TAU, time::Duration};

use bevy::prelude::*;

use crate::{
    game::GameState,
    screen::components::ScreenState,
};

const THRESHOLD_INCREMENT: u64 = 250;
pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BombDefusedEvent>()
            .insert_resource(MatchBomb::default())
            .add_systems(
                OnEnter(ScreenState::Game),
                (setup_countdown_display, setup_defused_count_display),
            )
            .add_systems(
                Update,
                (update_countdown_display, update_defused_count_display),
            )
            .add_systems(Update, explode_bomb.run_if(in_state(ScreenState::Game)))
            .add_systems(
                Update,
                (text_despawn_timer, rotate_bomb).run_if(in_state(ScreenState::Game)),
            )
            .add_systems(Update, on_defuse.run_if(on_event::<BombDefusedEvent>));
    }
}
#[derive(Resource)]
pub struct MatchBomb {
    turns_remaining: u64,
    point_threshold: u64,
    defused_count: u64,
}

impl Default for MatchBomb {
    fn default() -> Self {
        MatchBomb {
            turns_remaining: 5,
            point_threshold: THRESHOLD_INCREMENT,
            defused_count: 0,
        }
    }
}

impl MatchBomb {
    pub fn sub(&mut self, points: u64) {
        self.point_threshold = self.point_threshold.saturating_sub(points);
    }

    pub fn decrement(&mut self) {
        self.turns_remaining = self.turns_remaining.saturating_sub(1);
    }

    pub fn points_remaining(&self) -> u64 {
        self.point_threshold
    }

    pub fn turns_remaining(&self) -> u64 {
        self.turns_remaining
    }

    pub fn rearm(&mut self) {
        self.defused_count += 1;
        self.turns_remaining = 5;
        self.point_threshold = THRESHOLD_INCREMENT + (THRESHOLD_INCREMENT * self.defused_count);
    }

    pub fn reset(&mut self) {
        self.defused_count = 0;
        self.turns_remaining = 5;
        self.point_threshold = THRESHOLD_INCREMENT;
    }
}

#[derive(Component)]
pub struct BombPiece(pub (usize, usize));

impl BombPiece {
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.0.0 as f32, self.0.1 as f32)
    }
}

#[derive(Component)]
pub struct CountdownDisplay;

#[derive(Component)]
pub struct TextDespawn(Timer);

#[derive(Component)]
pub struct DefusedCountDisplay;

impl Default for TextDespawn {
    fn default() -> Self {
        TextDespawn(Timer::new(Duration::from_secs_f32(2.5), TimerMode::Once))
    }
}

#[derive(Event, Default)]
pub struct BombDefusedEvent;
//Systems

fn setup_defused_count_display(mut commads: Commands) {
    commads.spawn((
        Node {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            bottom: Val::Px(5.0),
            ..default()
        },
        DefusedCountDisplay,
        Text::new("0"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
    ));
}

fn setup_countdown_display(
    mut commands: Commands,
    bomb: Res<MatchBomb>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::VMin(0.0),
            align_self: AlignSelf::Center,
            width: Val::Px(50.0),
            ..default()
        },
        ImageNode {
            image: asset_server.load("image/bomb.png"),
            ..default()
        },
        Text::from(bomb.turns_remaining().to_string()),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
        CountdownDisplay,
    ));
}

fn update_countdown_display(
    mut commands: Commands,
    bomb: Res<MatchBomb>,
    display: Single<Entity, With<CountdownDisplay>>,
) {
    if bomb.is_changed() {
        commands
            .entity(*display)
            .insert(Text::from(bomb.turns_remaining().to_string()));
    }
}

fn update_defused_count_display(
    mut commands: Commands,
    display: Single<Entity, With<DefusedCountDisplay>>,
    bomb: Res<MatchBomb>,
) {
    if bomb.is_changed() {
        let display_text = if bomb.defused_count == 1 {
            format!("{} bomb defused", bomb.defused_count)
        } else {
            format!("{} bombs defused", bomb.defused_count)
        };
        commands.entity(*display).insert(Text::new(display_text));
    }
}
fn rotate_bomb(mut pieces_q: Query<&mut Transform, With<BombPiece>>, time: Res<Time>) {
    for mut piece in pieces_q.iter_mut() {
        piece.rotate_y(0.03 * TAU * time.delta_secs().sin());
        //        piece.rotate_z(0.03 * TAU * time.delta_secs().sin());
        piece.rotate_x(0.1 * TAU * time.delta_secs().sin());
    }
}

fn explode_bomb(bomb: Res<MatchBomb>, mut next_state: ResMut<NextState<GameState>>) {
    if bomb.turns_remaining == 0 && bomb.points_remaining() > 0 {
        next_state.set(GameState::GameOver)
    }
}

fn on_defuse(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mesh = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/defused_text.glb"));

    commands.spawn((
        SceneRoot(mesh),
        Transform::from_translation(Vec3::new(7.0, 7.0, 6.0)),
        TextDespawn::default(),
    ));
}

fn text_despawn_timer(
    mut commands: Commands,
    text_entity: Single<Entity, With<TextDespawn>>,
    mut timer: Single<&mut TextDespawn>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        commands.entity(*text_entity).despawn();
    }
}
