use bevy::{
  app::{App, Plugin, Update},
  ecs::{
    query::{Changed, With},
    schedule::{IntoScheduleConfigs, common_conditions::not},
    system::Single,
  },
  sprite::Sprite,
  state::condition::in_state,
  window::{PrimaryWindow, Window},
};

use crate::{components::grid::Grid, states::game_state::GameState};

pub struct GridPlugin;

impl Plugin for GridPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, resize.run_if(not(in_state(GameState::MainMenu))));
  }
}

fn resize(
  window: Single<&Window, (With<PrimaryWindow>, Changed<Window>)>,
  grid_query: Single<(&mut Grid, &mut Sprite)>,
) {
  let next_grid = Grid::from(window.into_inner());
  let (mut grid, mut sprite) = grid_query.into_inner();

  sprite.custom_size = Some(next_grid.size);
  *grid = next_grid;
}
