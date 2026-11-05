use bevy::{
  app::{App, Plugin, Update},
  ecs::{
    entity::Entity,
    observer::Trigger,
    query::With,
    schedule::{IntoScheduleConfigs, SystemSet},
    system::{Commands, Query, Res, ResMut, Single},
  },
  input::{ButtonInput, keyboard::KeyCode},
  state::{
    condition::in_state,
    state::{NextState, OnExit},
  },
  time::{Fixed, Time},
};

use crate::{
  components::{
    food::Food,
    grid::Grid,
    segment::Segment,
    snake::Snake,
    ui::{hud::Hud, overlay::Overlay},
  },
  events::game_events::EatFoodEvent,
  resources::{bonus::Bonus, level_up::LevelUp, score::Score},
  states::game_state::GameState,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InGameSystemSet;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .configure_sets(Update, InGameSystemSet)
      .add_systems(
        Update,
        (eat_food, pause, gameover)
          .run_if(in_state(GameState::InGame))
          .in_set(InGameSystemSet),
      )
      .add_systems(Update, unpause.run_if(in_state(GameState::Paused)))
      .add_systems(Update, restart.run_if(in_state(GameState::GameOver)))
      .add_systems(OnExit(GameState::GameOver), despawn_gameover)
      .add_observer(on_level_up);
  }
}

fn eat_food(
  snake_query: Single<&Snake>,
  food_query: Query<(Entity, &Food)>,
  mut score: ResMut<Score>,
  mut bonus: ResMut<Bonus>,
  mut level_up: ResMut<LevelUp>,
  mut commands: Commands,
) {
  let snake = snake_query.into_inner();

  food_query.iter().for_each(|(food_id, food)| {
    if snake.position == food.position {
      let eat_event = EatFoodEvent::new(food.variant);
      let points = food.points();

      score.value += points;
      bonus.progress += points;
      level_up.progress += points;
      commands.trigger_targets(eat_event, food_id);
    }
  });
}

fn pause(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
  if keyboard.into_inner().just_pressed(KeyCode::Escape) {
    next_state.set(GameState::Paused);
  }
}

fn unpause(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
  if keyboard.into_inner().just_pressed(KeyCode::Escape) {
    next_state.set(GameState::InGame);
  }
}

fn on_level_up(
  _trigger: Trigger<EatFoodEvent>,
  mut time: ResMut<Time<Fixed>>,
  mut level_up: ResMut<LevelUp>,
) {
  if level_up.progress >= level_up.threshold {
    let next_speed = time.timestep().as_secs_f64() * 0.8;

    time.set_timestep_seconds(next_speed);
    level_up.progress -= level_up.threshold;
    level_up.threshold = (level_up.threshold as f32 * 1.25).round() as u32;
  }
}

fn gameover(
  snake_query: Single<&Snake>,
  segment_query: Query<&Segment>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  let snake = snake_query.into_inner();
  let ate_tail = segment_query
    .iter()
    .any(|segment| segment.position == snake.position);

  if ate_tail {
    next_state.set(GameState::GameOver);
  }
}

fn restart(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
  if keyboard.into_inner().just_pressed(KeyCode::Enter) {
    next_state.set(GameState::MainMenu);
  }
}

#[allow(clippy::too_many_arguments)]
fn despawn_gameover(
  overlay: Single<Entity, With<Overlay>>,
  grid: Single<Entity, With<Grid>>,
  hud: Single<Entity, With<Hud>>,
  mut time: ResMut<Time<Fixed>>,
  mut bonus: ResMut<Bonus>,
  mut level_up: ResMut<LevelUp>,
  mut score: ResMut<Score>,
  mut commands: Commands,
) {
  let overlay_id = overlay.into_inner();
  let grid_id = grid.into_inner();
  let hud_id = hud.into_inner();

  commands.entity(overlay_id).despawn();
  commands.entity(grid_id).despawn();
  commands.entity(hud_id).despawn();
  time.set_timestep_seconds(LevelUp::INIT_SPEED);
  bonus.deactivate();
  level_up.reset();
  score.reset();
}
