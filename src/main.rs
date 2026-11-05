mod components;
mod events;
mod helpers;
mod plugins;
mod resources;
mod states;

use bevy::{
  DefaultPlugins,
  app::{App, PluginGroup},
  render::texture::ImagePlugin,
  state::app::AppExtStates,
};

use crate::{
  events::game_events::EatFoodEvent,
  plugins::{
    food_plugin::FoodPlugin, game_plugin::GamePlugin, grid_plugin::GridPlugin,
    hud_plugin::HudPlugin, menu_plugin::MenuPlugin, setup_plugin::SetupPlugin,
    snake_plugin::SnakePlugin,
  },
  states::game_state::GameState,
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(SetupPlugin)
    .add_plugins(MenuPlugin)
    .add_plugins(GridPlugin)
    .add_plugins(SnakePlugin)
    .add_plugins(FoodPlugin)
    .add_plugins(GamePlugin)
    .add_plugins(HudPlugin)
    .insert_state(GameState::MainMenu)
    .add_event::<EatFoodEvent>()
    .run();
}
