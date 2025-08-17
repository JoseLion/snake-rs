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
  math::Vec2,
  sprite::{Anchor, Sprite},
  transform::components::Transform,
  window::{PrimaryWindow, Window},
};

use crate::{
  components::{grid::Grid, segment::Segment, snake::Snake},
  helpers::theme::Theme,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Startup,
      (setup_camera, setup_grid, (setup_snake, setup_segments)).chain(),
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
    Transform::from_xyz(0.0, 0.0, 0.0),
  ));

  Ok(())
}

fn setup_segments(
  grid_query: Query<(Entity, &Grid)>,
  assets_res: Res<AssetServer>,
  mut commands: Commands,
) -> Result {
  let assets = assets_res.into_inner();
  let (entity, grid) = grid_query.single()?;

  commands.entity(entity).with_children(|parent| {
    (1..=3).for_each(|i| {
      parent.spawn((
        Segment::new(0, -i),
        Sprite {
          anchor: Anchor::TopLeft,
          custom_size: Some(Vec2::splat(grid.cell_size)),
          image: assets.load("snake/segment.png"),
          ..Sprite::default()
        },
        Transform::from_xyz(0.0, -grid.cell_size * i as f32, 0.0),
      ));
    });
  });

  Ok(())
}
