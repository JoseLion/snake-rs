mod components;
mod helpers;
mod plugins;

use bevy::{DefaultPlugins, app::App};

use crate::plugins::{
  grid_plugin::GridPlugin, setup_plugin::SetupPlugin, snake_plugin::SnakePlugin,
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(SetupPlugin)
    .add_plugins(GridPlugin)
    .add_plugins(SnakePlugin)
    .run();
}
