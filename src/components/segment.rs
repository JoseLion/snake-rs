use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
  asset::{AssetServer, Assets},
  ecs::{bundle::Bundle, component::Component},
  image::{TextureAtlas, TextureAtlasLayout},
  math::{I8Vec2, Quat, UVec2, Vec2},
  sprite::Sprite,
  transform::components::Transform,
};

use crate::components::grid::Grid;

#[derive(Clone, Copy, PartialEq)]
pub enum SegmentKind {
  Body,
  Corner,
  Tail,
}

#[derive(Component, Clone)]
#[require(Sprite)]
pub struct Segment {
  pub direction: I8Vec2,
  pub kind: SegmentKind,
  pub position: I8Vec2,
  pub grown: bool,
}

impl Segment {
  pub fn atlas_index(&self) -> usize {
    if self.grown {
      return match self.kind {
        SegmentKind::Body => 3,
        SegmentKind::Corner => 4,
        SegmentKind::Tail => 2,
      };
    }

    match self.kind {
      SegmentKind::Body => 0,
      SegmentKind::Corner => 1,
      SegmentKind::Tail => 2,
    }
  }

  pub fn rotation(&self, prev_direction: I8Vec2) -> Quat {
    let angle = match self.kind {
      SegmentKind::Corner => match prev_direction {
        I8Vec2::X => 0.0,
        I8Vec2::Y => FRAC_PI_2,
        I8Vec2::NEG_X => -PI,
        I8Vec2::NEG_Y => -FRAC_PI_2,
        _ => 0.0,
      },
      _ => match self.direction {
        I8Vec2::Y => FRAC_PI_2,
        I8Vec2::NEG_Y => -FRAC_PI_2,
        _ => 0.0,
      },
    };

    Quat::from_rotation_z(angle)
  }

  pub fn flip_x(&self) -> bool {
    self.kind != SegmentKind::Corner && self.direction == I8Vec2::NEG_X
  }

  pub fn flip_y(&self, prev_direction: I8Vec2) -> bool {
    self.kind == SegmentKind::Corner && prev_direction.perp_dot(self.direction) > 0
  }
}

#[derive(Bundle)]
pub struct SegmentBundle {
  segment: Segment,
  sprite: Sprite,
  transform: Transform,
}

impl SegmentBundle {
  pub fn new(
    segment: Segment,
    grid: &Grid,
    assets: &AssetServer,
    layout_asset: &mut Assets<TextureAtlasLayout>,
  ) -> Self {
    let image = assets.load("snake/body.png");
    let tile_size = UVec2::splat(32);
    let padding = Some(UVec2::splat(8));
    let texture_layout = TextureAtlasLayout::from_grid(tile_size, 5, 1, padding, None);
    let layout = layout_asset.add(texture_layout);
    let translation = grid.translation_at(&segment.position);
    let rotation = segment.rotation(segment.direction);
    let index = segment.atlas_index();

    SegmentBundle {
      segment,
      sprite: Sprite {
        custom_size: Some(Vec2::splat(grid.cell_size)),
        image,
        texture_atlas: Some(TextureAtlas { layout, index }),
        ..Sprite::default()
      },
      transform: Transform::from_translation(translation).with_rotation(rotation),
    }
  }
}
