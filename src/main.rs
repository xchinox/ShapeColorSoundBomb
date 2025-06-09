use crate::audio_server::*;
use crate::camera::*;
use crate::screen::*;
use crate::ui_theme::*;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::window::WindowResolution;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::*;
use game::GamePlugin;

pub mod audio_server;
pub mod camera;
pub mod game;
pub mod screen;
pub mod ui_theme;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ShapeColorSoundBomb".to_string(),
                        resolution: WindowResolution::new(1280.0, 720.0)
                            .with_scale_factor_override(1.0),
                        resizable: true,
                        mode: WindowMode::Windowed,
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(CameraPlugin)
        .add_plugins(ScreenPlugin)
        .add_plugins(AudioServerPlugin)
        // .add_plugins(EguiPlugin {
        //     enable_multipass_for_primary_context: true,
        // })
        // .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(UiThemePlugin)
        .add_plugins(GamePlugin)
        .add_plugins(MeshPickingPlugin)
        .run();
}
