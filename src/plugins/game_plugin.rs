use bevy::{
  app::{App, Plugin, Update},
  ecs::{
    entity::Entity,
    observer::Trigger,
    system::{Commands, Query, ResMut, Single},
  },
  time::{Fixed, Time},
};

use crate::{
  components::{food::Food, snake::Snake},
  events::game_events::EatFoodEvent,
  resources::{bonus::Bonus, level_up::LevelUp, score::Score},
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, eat_food).add_observer(on_level_up);
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
