use std::f32::consts::FRAC_PI_2;

use bevy::{
  ecs::component::Component,
  math::{I8Vec2, Quat},
  sprite::Sprite,
};

use crate::helpers::transform::{Flip, SpriteTx};

#[derive(Component)]
#[require(Sprite)]
pub struct Snake {
  pub direction: I8Vec2,
  pub has_food: bool,
  pub position: I8Vec2,
}

impl Default for Snake {
  fn default() -> Self {
    Snake {
      direction: I8Vec2::Y,
      has_food: false,
      position: I8Vec2::ZERO,
    }
  }
}

impl Snake {
  pub const IDLE: usize = 0;
  pub const OPEN: usize = 1;

  pub fn sprite_tx(&self) -> SpriteTx {
    let angle = match self.direction {
      I8Vec2::X => 0.0,
      I8Vec2::Y => FRAC_PI_2,
      I8Vec2::NEG_X => 0.0,
      I8Vec2::NEG_Y => -FRAC_PI_2,
      _ => 0.0,
    };

    SpriteTx {
      rotation: Quat::from_rotation_z(angle),
      flip: Flip {
        x: self.direction == I8Vec2::NEG_X,
        y: false,
      },
    }
  }
}
