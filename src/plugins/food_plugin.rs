use std::iter::once;

use bevy::{
  app::{App, Plugin, Update},
  asset::AssetServer,
  ecs::{
    entity::Entity,
    observer::Trigger,
    query::Changed,
    system::{Commands, Query, Res, ResMut, Single},
  },
  math::Vec2,
  platform::collections::HashSet,
  sprite::Sprite,
  time::Time,
  transform::components::Transform,
};

use crate::{
  components::{
    food::{Food, FoodBundle, FoodVariant},
    grid::Grid,
    segment::Segment,
    snake::Snake,
  },
  events::game_events::EatFoodEvent,
  resources::bonus::Bonus,
};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (resize, exhaust_bonus))
      .add_observer(on_respawn)
      .add_observer(on_spawn_bonus);
  }
}

fn resize(
  grid_query: Single<&Grid, Changed<Grid>>,
  mut food_query: Query<(&Food, &mut Sprite, &mut Transform)>,
) {
  let grid = grid_query.into_inner();

  food_query
    .iter_mut()
    .for_each(|(food, mut food_sprite, mut food_tx)| {
      food_sprite.custom_size = Some(Vec2::splat(grid.cell_size));
      food_tx.translation = grid.translation_at(&food.position);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn on_respawn(
  trigger: Trigger<EatFoodEvent>,
  grid_query: Single<&Grid>,
  snake_query: Single<&Snake>,
  segment_query: Query<&Segment>,
  mut food_query: Query<(&mut Food, &mut Transform)>,
  mut bonus: ResMut<Bonus>,
  mut commands: Commands,
) {
  let grid = grid_query.into_inner();
  let snake = snake_query.into_inner();
  let food_positions = food_query
    .iter()
    .map(|(f, _)| f.position)
    .collect::<HashSet<_>>();

  if let Ok((mut food, mut food_tx)) = food_query.get_mut(trigger.target()) {
    match food.variant {
      FoodVariant::Normal => {
        let excludes = once(snake.position)
          .chain(segment_query.iter().map(|s| s.position))
          .chain(food_positions)
          .collect::<HashSet<_>>();
        let next_position = grid.random_position(&excludes);

        food.position = next_position;
        food_tx.translation = grid.translation_at(&food.position);
      }
      FoodVariant::Special => {
        bonus.deactivate();
        commands.entity(trigger.target()).despawn();
      }
    }
  }
}

#[allow(clippy::too_many_arguments)]
fn on_spawn_bonus(
  _trigger: Trigger<EatFoodEvent>,
  grid_query: Single<(Entity, &Grid)>,
  snake_query: Single<&Snake>,
  segment_query: Query<&Segment>,
  food_query: Query<&Food>,
  assets_res: Res<AssetServer>,
  mut bonus: ResMut<Bonus>,
  mut commands: Commands,
) {
  if bonus.is_ready() {
    let assets = assets_res.into_inner();
    let (grid_id, grid) = grid_query.into_inner();
    let snake = snake_query.into_inner();
    let excludes = once(snake.position)
      .chain(segment_query.iter().map(|s| s.position))
      .chain(food_query.iter().map(|f| f.position))
      .collect::<HashSet<_>>();
    let position = grid.random_position(&excludes);

    bonus.activate();
    commands.entity(grid_id).with_child(FoodBundle::from(
      FoodVariant::Special,
      position,
      grid,
      assets,
    ));
  }
}

fn exhaust_bonus(
  food_query: Query<(Entity, &Food)>,
  time_res: Res<Time>,
  mut bonus: ResMut<Bonus>,
  mut commands: Commands,
) {
  if bonus.active {
    let time = time_res.into_inner();

    bonus.timer.tick(time.delta());

    if bonus.timer.finished() {
      bonus.deactivate();

      food_query
        .iter()
        .filter(|(_, f)| f.variant == FoodVariant::Special)
        .for_each(|(food_id, _)| commands.entity(food_id).despawn());
    }
  }
}
