use bevy::{
  app::{App, FixedUpdate, Plugin, Update},
  asset::{AssetServer, Assets},
  ecs::{
    entity::Entity,
    event::EventReader,
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
  resources::timers::MouthTimer,
};

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        Update,
        (resize_snake, update_snake, grow_on_eat, open_mouth),
      )
      .add_systems(FixedUpdate, (move_snake, move_segments))
      .insert_resource(Time::<Fixed>::from_seconds(0.25))
      .init_resource::<MouthTimer>();
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
  snake_tx.translation = grid.translation_at(&snake.position);
  snake_tx.rotation = snake.rotation();
  snake_sprite.flip_x = snake.flip_x();
}

#[allow(clippy::needless_pass_by_value)]
fn move_segments(
  grid_query: Single<&Grid>,
  eat_event: EventReader<EatFoodEvent>,
  snake: Single<&mut Snake, Changed<Snake>>,
  mut segment_query: Query<(&mut Segment, &mut Transform, &mut Sprite)>,
) {
  let grid = grid_query.into_inner();
  let segment_len = segment_query.iter().len();
  let head_segment = Segment {
    direction: snake.direction,
    kind: SegmentKind::Body,
    position: snake.position,
    grown: !eat_event.is_empty(),
  };

  segment_query
    .iter_mut()
    .enumerate()
    .fold(head_segment, |acc, (i, current)| {
      let (mut segment, mut segment_tx, mut segment_spr) = current;
      let prev = segment.clone();

      segment.position = acc.position;
      segment.direction = acc.direction;
      segment.grown = acc.grown;
      segment.kind = match i {
        idx if idx == segment_len - 1 => SegmentKind::Tail,
        _ if segment.direction != prev.direction => SegmentKind::Corner,
        _ => acc.kind,
      };

      segment_tx.translation = grid.translation_at(&segment.position);
      segment_tx.rotation = segment.rotation(prev.direction);

      if let Some(atlas) = segment_spr.texture_atlas.as_mut() {
        atlas.index = segment.atlas_index();
        segment_spr.flip_x = segment.flip_x();
        segment_spr.flip_y = segment.flip_y(prev.direction);
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

fn grow_on_eat(
  grid_query: Single<(Entity, &Grid)>,
  segment_query: Query<&Segment>,
  assets_res: Res<AssetServer>,
  mut layout_asset: ResMut<Assets<TextureAtlasLayout>>,
  mut event_reader: EventReader<EatFoodEvent>,
  mut commands: Commands,
) {
  if !event_reader.is_empty() {
    let (grid_id, grid) = grid_query.into_inner();
    let assets = assets_res.into_inner();

    event_reader.read().for_each(|eat_event| {
      let should_grow = eat_event.variant == FoodVariant::Normal;

      if should_grow && let Some(last) = segment_query.iter().last() {
        let segment = Segment {
          direction: last.direction,
          kind: SegmentKind::Tail,
          position: last.position,
          grown: false,
        };

        commands.entity(grid_id).with_child(SegmentBundle::new(
          segment,
          grid,
          assets,
          &mut layout_asset,
          last.direction,
        ));
      }
    });
  }
}

fn open_mouth(
  grid_query: Single<&Grid>,
  snake_query: Single<(&Snake, &mut Sprite)>,
  food_query: Query<&Food>,
  time_res: Res<Time>,
  mut mouth_timer: ResMut<MouthTimer>,
) {
  let grid = grid_query.into_inner();
  let (snake, mut snake_sprite) = snake_query.into_inner();
  let next_position = grid.warp(snake.position + snake.direction);
  let should_open = food_query
    .iter()
    .any(|food| next_position == food.position || snake.position == food.position);

  if let Some(atlas) = snake_sprite.texture_atlas.as_mut() {
    if atlas.index == Snake::OPEN {
      let timer = time_res.into_inner();
      mouth_timer.value.tick(timer.delta());
    }

    if should_open {
      atlas.index = Snake::OPEN;
    } else if mouth_timer.value.finished() {
      atlas.index = Snake::IDLE;
      mouth_timer.value.reset();
    }
  }
}
