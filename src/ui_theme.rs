use bevy::prelude::*;

pub struct UiThemePlugin;

impl Plugin for UiThemePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

const BUTTON_BACKGROUND_HOVER: Color = Color::srgb(1.85, 0.15, 1.0);
const BUTTON_BACKGROUND_NORMAL: Color = Color::srgb(0.0, 0.0, 0.0);
const BUTTON_BACKGROUND_PRESSED: Color = Color::srgb(0.95, 0.95, 0.95);

const BUTTON_BORDER_HOVER: Color = Color::srgb(0.15, 0.15, 0.15);
const BUTTON_BORDER_NORMAL: Color = Color::srgb(0.25, 0.25, 0.25);
const BUTTON_BORDER_PRESSED: Color = Color::srgb(0.55, 0.55, 0.55);

const TEXT_FONT_PATH: &str = "fonts/OpenDyslexic-Regular.otf";

#[derive(Resource)]
pub struct UiTheme {
    pub button_background_hover: Color,
    pub button_background_normal: Color,
    pub button_background_pressed: Color,

    pub button_border_hover: Color,
    pub button_border_normal: Color,
    pub button_border_pressed: Color,

    text_font: TextFont,
}
impl UiTheme {
    fn set_font(&mut self, font: TextFont) -> &mut Self {
        self.text_font = font;
        self
    }

    pub fn font(&self) -> TextFont {
        self.text_font.clone()
    }
}
impl Default for UiTheme {
    fn default() -> Self {
        UiTheme {
            button_background_hover: BUTTON_BACKGROUND_HOVER,
            button_background_normal: BUTTON_BACKGROUND_NORMAL,
            button_background_pressed: BUTTON_BACKGROUND_PRESSED,
            button_border_hover: BUTTON_BORDER_HOVER,
            button_border_normal: BUTTON_BORDER_NORMAL,
            button_border_pressed: BUTTON_BORDER_PRESSED,
            text_font: TextFont { ..default() },
        }
    }
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font_handle: Handle<Font> = asset_server.load(TEXT_FONT_PATH);
    let text_font = TextFont {
        font: font_handle,
        font_size: 12.0,
        font_smoothing: bevy::text::FontSmoothing::AntiAliased,

        ..default()
    };

    let mut theme = UiTheme::default();
    theme.set_font(text_font);

    commands.insert_resource(theme);
}
