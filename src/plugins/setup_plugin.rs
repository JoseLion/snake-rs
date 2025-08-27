use std::iter::once;

use bevy::{
  app::{App, Plugin, Startup},
  asset::AssetServer,
  core_pipeline::core_2d::Camera2d,
  ecs::{
    entity::Entity,
    error::Result,
    query::With,
    schedule::IntoScheduleConfigs,
    system::{Commands, Query, Res},
  },
  math::{I8Vec2, Vec2},
  platform::collections::HashSet,
  sprite::{Anchor, Sprite},
  transform::components::Transform,
  window::{PrimaryWindow, Window},
};

use crate::{
  components::{
    food::{FoodBundle, FoodVariant},
    grid::Grid,
    segment::{Segment, SegmentBundle},
    snake::Snake,
  },
  helpers::theme::Theme,
  resources::{bonus::Bonus, level_up::LevelUp, score::Score},
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<Score>()
      .init_resource::<LevelUp>()
      .init_resource::<Bonus>()
      .add_systems(
        Startup,
        (
          setup_camera,
          setup_grid,
          (setup_snake, setup_segments),
          setup_food,
        )
          .chain(),
      );
  }
}

fn setup_camera(mut commands: Commands) {
  commands.spawn(Camera2d);
}

fn setup_grid(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) -> Result {
  let window = window_query.single()?;
  let grid = Grid::from(window);
  let sprite = Sprite {
    color: Theme::WORLD_COLOR,
    custom_size: Some(grid.size),
    ..Default::default()
  };

  commands.spawn((grid, sprite));

  Ok(())
}

fn setup_snake(
  grid_query: Query<(Entity, &Grid)>,
  assets: Res<AssetServer>,
  mut commands: Commands,
) -> Result {
  let (grid_id, grid) = grid_query.single()?;

  commands.entity(grid_id).with_child((
    Snake::default(),
    Sprite {
      anchor: Anchor::TopLeft,
      custom_size: Some(Vec2::splat(grid.cell_size)),
      image: assets.into_inner().load("snake/segment.png"),
      ..Sprite::default()
    },
    Transform::default(),
  ));

  Ok(())
}

fn setup_segments(
  grid_query: Query<(Entity, &Grid)>,
  assets_res: Res<AssetServer>,
  mut commands: Commands,
) -> Result {
  let assets = assets_res.into_inner();
  let (grid_id, grid) = grid_query.single()?;

  commands.entity(grid_id).with_children(|parent| {
    (1..=3).for_each(|i| {
      let position = I8Vec2::new(0, -i);

      parent.spawn(SegmentBundle::from(grid, &position, assets));
    });
  });

  Ok(())
}

fn setup_food(
  grid_query: Query<&Grid>,
  snake_query: Query<&Snake>,
  segment_query: Query<&Segment>,
  assets_res: Res<AssetServer>,
  mut commands: Commands,
) -> Result {
  let assets = assets_res.into_inner();
  let grid = grid_query.single()?;
  let snake = snake_query.single()?;
  let excludes = once(snake.position)
    .chain(segment_query.iter().map(|s| s.position))
    .collect::<HashSet<_>>();
  let position = grid.random_position(&excludes)?;

  commands.spawn(FoodBundle::from(
    FoodVariant::Normal,
    position,
    grid,
    assets,
  ));

  Ok(())
}
