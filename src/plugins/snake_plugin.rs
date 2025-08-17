use bevy::{
  app::{App, FixedUpdate, Plugin, Update},
  ecs::{
    query::{Changed, Without},
    system::{Query, Res, Single},
  },
  input::{ButtonInput, keyboard::KeyCode},
  math::{I8Vec2, Vec2},
  sprite::Sprite,
  time::{Fixed, Time},
  transform::components::Transform,
};

use crate::components::{grid::Grid, segment::Segment, snake::Snake};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (resize_snake, update_snake))
      .add_systems(FixedUpdate, (move_snake, move_segments))
      .insert_resource(Time::<Fixed>::from_seconds(0.25));
  }
}

fn resize_snake(
  grid_query: Single<&Grid, Changed<Grid>>,
  snake_query: Single<(&Snake, &mut Sprite, &mut Transform), Without<Segment>>,
  mut segment_query: Query<(&Segment, &mut Sprite, &mut Transform), Without<Snake>>,
) {
  let grid = grid_query.into_inner();
  let (snake, mut snake_sprite, mut snake_tx) = snake_query.into_inner();

  snake_sprite.custom_size = Some(Vec2::splat(grid.cell_size));
  snake_tx.translation = grid.translation_at(snake.position);

  segment_query
    .iter_mut()
    .for_each(|(segment, mut segment_sprite, mut segment_tx)| {
      segment_sprite.custom_size = Some(Vec2::splat(grid.cell_size));
      segment_tx.translation = grid.translation_at(segment.position);
    });
}

fn move_snake(grid_query: Single<&Grid>, snake_query: Single<(&mut Snake, &mut Transform)>) {
  let grid = grid_query.into_inner();
  let (mut snake, mut snake_tx) = snake_query.into_inner();

  snake.position = grid.warp(snake.position + snake.direction);
  snake_tx.translation = grid.translation_at(snake.position);
}

fn move_segments(
  grid_query: Single<&Grid>,
  snake_query: Single<&Snake, Changed<Snake>>,
  mut segment_query: Query<(&mut Segment, &mut Transform)>,
) {
  let grid = grid_query.into_inner();
  let snake = snake_query.into_inner();
  let positions = segment_query
    .iter_mut()
    .map(|(segment, _)| segment.position)
    .collect::<Vec<_>>();

  segment_query
    .iter_mut()
    .enumerate()
    .for_each(|(i, (mut segment, mut segment_tx))| {
      if i == 0 {
        segment.position = snake.position;
        segment_tx.translation = grid.translation_at(snake.position);
      } else {
        segment.position = positions[i - 1];
        segment_tx.translation = grid.translation_at(segment.position);
      }
    });
}

fn update_snake(
  keyboard: Res<ButtonInput<KeyCode>>,
  grid_query: Single<&Grid>,
  segment_query: Query<&Segment>,
  mut snake: Single<&mut Snake>,
) {
  let grid = grid_query.into_inner();

  if let Some(first_segment) = segment_query.iter().next() {
    keyboard
      .into_inner()
      .get_just_pressed()
      .for_each(|key_code| {
        let next_direction = match key_code {
          KeyCode::ArrowUp => I8Vec2::Y,
          KeyCode::ArrowDown => I8Vec2::NEG_Y,
          KeyCode::ArrowLeft => I8Vec2::NEG_X,
          KeyCode::ArrowRight => I8Vec2::X,
          _ => return,
        };
        let next_position = grid.warp(snake.position + next_direction);

        if next_position != first_segment.position {
          snake.direction = next_direction;
        }
      });
  }
}
