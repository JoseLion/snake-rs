mod components;
mod events;
mod helpers;
mod plugins;
mod resources;

use bevy::{
  DefaultPlugins,
  app::{App, PluginGroup},
  render::texture::ImagePlugin,
};

use crate::{
  events::game_events::EatFoodEvent,
  plugins::{
    food_plugin::FoodPlugin, game_plugin::GamePlugin, grid_plugin::GridPlugin,
    setup_plugin::SetupPlugin, snake_plugin::SnakePlugin,
  },
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugins(SetupPlugin)
    .add_plugins(GridPlugin)
    .add_plugins(SnakePlugin)
    .add_plugins(FoodPlugin)
    .add_plugins(GamePlugin)
    .add_event::<EatFoodEvent>()
    .run();
}
