use std::iter::once;

use bevy::{
  app::{App, Plugin, Startup},
  asset::{AssetServer, Assets},
  core_pipeline::core_2d::Camera2d,
  ecs::{
    entity::Entity,
    error::Result,
    query::With,
    schedule::IntoScheduleConfigs,
    system::{Commands, Query, Res, ResMut},
  },
  image::{TextureAtlas, TextureAtlasLayout},
  math::{I8Vec2, UVec2, Vec2},
  platform::collections::HashSet,
  sprite::Sprite,
  transform::components::Transform,
  window::{PrimaryWindow, Window},
};

use crate::{
  components::{
    food::{FoodBundle, FoodVariant},
    grid::Grid,
    segment::{Segment, SegmentBundle, SegmentKind},
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
  assets_res: Res<AssetServer>,
  mut layout_asset: ResMut<Assets<TextureAtlasLayout>>,
  mut commands: Commands,
) -> Result {
  let (grid_id, grid) = grid_query.single()?;
  let assets = assets_res.into_inner();
  let image = assets.load("snake/head.png");
  let tile_size = UVec2::splat(32);
  let padding = Some(UVec2::splat(8));
  let texture_layout = TextureAtlasLayout::from_grid(tile_size, 2, 1, padding, None);
  let layout = layout_asset.add(texture_layout);
  let snake = Snake::default();
  let position = snake.position;
  let rotation = snake.rotation();

  commands.entity(grid_id).with_child((
    snake,
    Sprite {
      custom_size: Some(Vec2::splat(grid.cell_size)),
      image,
      texture_atlas: Some(TextureAtlas { layout, index: 0 }),
      ..Sprite::default()
    },
    Transform::from_translation(grid.translation_at(&position)).with_rotation(rotation),
  ));

  Ok(())
}

fn setup_segments(
  grid_query: Query<(Entity, &Grid)>,
  assets_res: Res<AssetServer>,
  mut layout_asset: ResMut<Assets<TextureAtlasLayout>>,
  mut commands: Commands,
) -> Result {
  let assets = assets_res.into_inner();
  let (grid_id, grid) = grid_query.single()?;

  commands.entity(grid_id).with_children(|parent| {
    (1..=4).for_each(|i| {
      let segment = Segment {
        direction: I8Vec2::Y,
        kind: match i {
          4 => SegmentKind::Tail,
          _ => SegmentKind::Body,
        },
        position: I8Vec2::new(0, -i),
        has_food: false,
      };

      parent.spawn(SegmentBundle::new(
        segment,
        grid,
        assets,
        &mut layout_asset,
        I8Vec2::Y,
      ));
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
  let position = grid.random_position(&excludes);

  commands.spawn(FoodBundle::from(
    FoodVariant::Normal,
    position,
    grid,
    assets,
  ));

  Ok(())
}
