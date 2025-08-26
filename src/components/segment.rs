use bevy::{
  asset::AssetServer,
  ecs::{bundle::Bundle, component::Component},
  math::{I8Vec2, Vec2},
  sprite::{Anchor, Sprite},
  transform::components::Transform,
};

use crate::components::grid::Grid;

#[derive(Component)]
#[require(Sprite)]
pub struct Segment {
  pub position: I8Vec2,
}

impl Segment {
  pub fn new(x: i8, y: i8) -> Self {
    Segment {
      position: I8Vec2::new(x, y),
    }
  }

  pub fn from(position: &I8Vec2) -> Self {
    Segment::new(position.x, position.y)
  }
}

#[derive(Bundle)]
pub struct SegmentBundle {
  segment: Segment,
  sprite: Sprite,
  transform: Transform,
}

impl SegmentBundle {
  pub fn from(grid: &Grid, position: &I8Vec2, assets: &AssetServer) -> Self {
    SegmentBundle {
      segment: Segment::from(position),
      sprite: Sprite {
        anchor: Anchor::TopLeft,
        custom_size: Some(Vec2::splat(grid.cell_size)),
        image: assets.load("snake/segment.png"),
        ..Sprite::default()
      },
      transform: Transform::from_translation(grid.translation_at(position)),
    }
  }
}
