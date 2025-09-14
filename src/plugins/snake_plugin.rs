use bevy::{
  app::{App, FixedUpdate, Plugin, Update},
  asset::{AssetServer, Assets},
  ecs::{
    entity::Entity,
    observer::Trigger,
    query::{Changed, Without},
    system::{Commands, Query, Res, ResMut, Single},
  },
  image::TextureAtlasLayout,
  input::{ButtonInput, keyboard::KeyCode},
  math::{I8Vec2, Vec2},
  sprite::Sprite,
  time::{Fixed, Time},
  transform::components::Transform,
};

use crate::{
  components::{
    food::{Food, FoodVariant},
    grid::Grid,
    segment::{Segment, SegmentBundle, SegmentKind},
    snake::Snake,
  },
  events::game_events::EatFoodEvent,
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (resize_snake, update_snake, open_mouth))
      .add_systems(FixedUpdate, (move_snake, move_segments))
      .add_observer(on_eat_food)
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
  snake_tx.translation = grid.translation_at(&snake.position);

  segment_query
    .iter_mut()
    .for_each(|(segment, mut segment_sprite, mut segment_tx)| {
      segment_sprite.custom_size = Some(Vec2::splat(grid.cell_size));
      segment_tx.translation = grid.translation_at(&segment.position);
    });
}

fn move_snake(
  grid_query: Single<&Grid>,
  snake_query: Single<(&mut Snake, &mut Transform, &mut Sprite)>,
) {
  let grid = grid_query.into_inner();
  let (mut snake, mut snake_tx, mut snake_sprite) = snake_query.into_inner();
  snake.position = grid.warp(snake.position + snake.direction);

  let sprite_tx = snake.sprite_tx();
  snake_tx.translation = grid.translation_at(&snake.position);
  snake_tx.rotation = sprite_tx.rotation;
  snake_sprite.flip_x = sprite_tx.flip.x;

  snake.has_food = false;
}

fn move_segments(
  grid_query: Single<&Grid>,
  snake_query: Single<&mut Snake, Changed<Snake>>,
  mut segment_query: Query<(&mut Segment, &mut Transform, &mut Sprite)>,
) {
  let grid = grid_query.into_inner();
  let snake = snake_query.into_inner();
  let segment_len = segment_query.iter().len();
  let head_segment = Segment {
    direction: snake.direction,
    kind: SegmentKind::Body,
    position: snake.position,
    has_food: snake.has_food,
  };

  segment_query
    .iter_mut()
    .enumerate()
    .fold(head_segment, |acc, (i, current)| {
      let (mut segment, mut segment_tx, mut segment_spr) = current;
      let prev = segment.clone();

      segment.position = acc.position;
      segment.direction = acc.direction;
      segment.has_food = acc.has_food;
      segment.kind = match i {
        idx if idx == segment_len - 1 => SegmentKind::Tail,
        _ if segment.direction != prev.direction => SegmentKind::Corner,
        _ => acc.kind,
      };

      let sprite_tx = segment.sprite_tx(prev.direction);
      segment_tx.translation = grid.translation_at(&segment.position);
      segment_tx.rotation = sprite_tx.rotation;

      if let Some(atlas) = segment_spr.texture_atlas.as_mut() {
        atlas.index = segment.atlas_index();
        segment_spr.flip_x = sprite_tx.flip.x;
        segment_spr.flip_y = sprite_tx.flip.y;
      }

      prev
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

#[allow(clippy::needless_pass_by_value)]
fn on_eat_food(
  trigger: Trigger<EatFoodEvent>,
  grid_query: Single<(Entity, &Grid)>,
  segment_query: Query<&Segment>,
  assets_res: Res<AssetServer>,
  mut snake: Single<&mut Snake>,
  mut layout_asset: ResMut<Assets<TextureAtlasLayout>>,
  mut commands: Commands,
) {
  let (grid_id, grid) = grid_query.into_inner();
  let assets = assets_res.into_inner();
  let should_grow = trigger.event().variant == FoodVariant::Normal;

  snake.has_food = true;

  if should_grow && let Some(last) = segment_query.iter().last() {
    let segment = Segment {
      direction: last.direction,
      kind: SegmentKind::Tail,
      position: last.position,
      has_food: false,
    };

    commands.entity(grid_id).with_child(SegmentBundle::new(
      segment,
      grid,
      assets,
      &mut layout_asset,
      last.direction,
    ));
  }
}

fn open_mouth(
  grid_query: Single<&Grid>,
  snake_query: Single<(&Snake, &mut Sprite)>,
  food_query: Query<&Food>,
) {
  let grid = grid_query.into_inner();
  let (snake, mut snake_sprite) = snake_query.into_inner();
  let next_position = grid.warp(snake.position + snake.direction);
  let should_open = food_query
    .iter()
    .any(|food| next_position == food.position || snake.has_food);

  if let Some(atlas) = snake_sprite.texture_atlas.as_mut() {
    atlas.index = match should_open {
      true => Snake::OPEN,
      false => Snake::IDLE,
    };
  }
}
