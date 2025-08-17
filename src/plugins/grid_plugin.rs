use bevy::{
  app::{App, Plugin, Update},
  ecs::{
    query::{Changed, With},
    system::Single,
  },
  sprite::Sprite,
  window::{PrimaryWindow, Window},
};

use crate::components::grid::Grid;

pub struct GridPlugin;

impl Plugin for GridPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, resize_grid);
  }
}

fn resize_grid(
  window: Single<&Window, (With<PrimaryWindow>, Changed<Window>)>,
  grid_query: Single<(&mut Grid, &mut Sprite)>,
) {
  let next_grid = Grid::from(window.into_inner());
  let (mut grid, mut sprite) = grid_query.into_inner();

  sprite.custom_size = Some(next_grid.size);
  *grid = next_grid;
}
