use bevy::color::Color;

pub struct Theme;

impl Theme {
  pub const MENU_IDLE: Color = Color::srgb_u8(200, 200, 200);
  pub const MENU_SELECTED: Color = Color::srgb_u8(255, 100, 100);
  pub const OVERLAY: Color = Color::srgba_u8(43, 44, 47, 190);
  pub const WORLD_COLOR: Color = Color::srgb_u8(173, 204, 96);
}
