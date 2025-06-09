use bevy::prelude::*;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum ScreenState {
    #[default]
    Splash,
    Menu,
    Option,
    Game,
    Credits,
    Exit
}

#[derive(Component)]
pub struct SplashTimer {
    pub timer: Timer
}


//Splash screen sprite marker
#[derive(Component)]
pub struct SplashSprite;