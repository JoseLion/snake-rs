use bevy::color::Color;

pub struct Theme;

impl Theme {
  pub const MENU_IDLE: Color = Color::srgb_u8(200, 200, 200);
  pub const MENU_SELECTED: Color = Color::srgb_u8(255, 100, 100);
  pub const OVERLAY: Color = Color::srgba_u8(43, 44, 47, 190);
  pub const WORLD_COLOR: Color = Color::srgb_u8(173, 204, 96);
}

pub struct FontFace;

impl FontFace {
  pub const PIXELIFY_SANS: &'static str = "fonts/PixelifySans-Variable.ttf";
  pub const PRESS_START_2P: &'static str = "fonts/PressStart2P-Regular.ttf";
}

pub struct FontSize;

impl FontSize {
  pub const BTN: f32 = 20.0;
  pub const H1: f32 = 24.0;
  pub const H2: f32 = 20.0;
  pub const H3: f32 = 16.0;
}
