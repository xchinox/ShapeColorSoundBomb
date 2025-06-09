use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    prelude::*,
};

use crate::screen::components::ScreenState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(OnEnter(ScreenState::Game), swap_camera_system);
    }
}

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera));
}

#[derive(Component)]
pub struct TextCamera;

fn swap_camera_system(mut commands: Commands, camera: Single<Entity, With<MainCamera>>) {
    commands.entity(*camera).despawn();

    commands.spawn((
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
        Transform::from_translation(Vec3::new(7.0, 8.0, 12.0)).looking_to(Vec3::ZERO, Dir3::Y),
        DirectionalLight {
            illuminance: 500.0,
            ..default()
        },
        MainCamera,
    ));
}

///Marker component
#[derive(Component)]
pub struct GameBoard;
