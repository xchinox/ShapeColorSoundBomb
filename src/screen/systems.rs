use std::time::Duration;

use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};

use super::components::{ScreenState, SplashSprite, SplashTimer};

const SPLASH_DURATION: f32 = 5.0;

pub fn setup_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("bevy_logo_dark.png"),
            // custom_size: Some(Vec2::new(640.0, 480.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.5, 0.5, 0.5)),
        SplashSprite,
    ));

    commands.spawn(SplashTimer {
        timer: Timer::from_seconds(SPLASH_DURATION, TimerMode::Once),
    });
}

pub fn continue_from_splash(
    mut splash: Single<&mut SplashTimer>,
    mut next_state: ResMut<NextState<ScreenState>>,
    time: Res<Time>,
    mut evt_kbd: EventReader<KeyboardInput>,
) {
    let splash_duration = if cfg!(debug_assertions) {
        Duration::from_secs_f32(0.0)
    } else {
        Duration::from_secs_f32(2.5)
    };

    if splash.timer.elapsed() > splash_duration {
        for evt in evt_kbd.read() {
            match evt.state {
                ButtonState::Pressed => continue,
                ButtonState::Released => {
                    splash.timer.pause();
                    next_state.set(ScreenState::Menu);
                }
            }
        }
    }
    if splash.timer.finished() {
        next_state.set(ScreenState::Menu);
        info!("Splash Timer Finished");
    } else {
        splash.timer.tick(time.delta());
    }
}

pub fn deconstruct_splash(
    mut commands: Commands,
    sprite_q: Single<Entity, With<SplashSprite>>,
    timer: Single<Entity, With<SplashTimer>>,
) {
    info!("Deconstructing Splash");
    commands.entity(sprite_q.into_inner()).despawn();
    commands.entity(*timer).despawn();
}
