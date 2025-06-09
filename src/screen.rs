use bevy::prelude::*;

use components::*;
use menu::MainMenuPlugin;
use credits::CreditsPlugin;
use options::OptionsPlugin;
use systems::*;



pub mod components;
pub mod systems;
pub mod menu;
pub mod credits;
pub mod options;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<ScreenState>()
        .add_systems(Startup, setup_splash_screen)
        .add_systems(Update, continue_from_splash.run_if(in_state(ScreenState::Splash)))
        .add_systems(OnExit(ScreenState::Splash), deconstruct_splash)
        .add_plugins(MainMenuPlugin)
        .add_plugins(CreditsPlugin)        
        .add_plugins(OptionsPlugin)
        ;
    }
}



