use std::f64::consts::FRAC_PI_2;

use bevy::{
  ecs::component::Component,
  math::{I8Vec2, Quat},
  sprite::Sprite,
};

#[derive(Component)]
#[require(Sprite)]
pub struct Snake {
  pub direction: I8Vec2,
  pub position: I8Vec2,
}

impl Default for Snake {
  fn default() -> Self {
    Snake {
      direction: I8Vec2::Y,
      position: I8Vec2::ZERO,
    }
  }
}

impl Snake {
  pub const IDLE: usize = 0;
  pub const OPEN: usize = 1;

  pub fn rotation(&self) -> Quat {
    let angle = match self.direction {
      I8Vec2::X => 0.0,
      I8Vec2::Y => FRAC_PI_2,
      I8Vec2::NEG_X => 0.0,
      I8Vec2::NEG_Y => -FRAC_PI_2,
      _ => 0.0,
    };

    Quat::from_rotation_z(angle as f32)
  }

  pub fn flip_x(&self) -> bool {
    self.direction == I8Vec2::NEG_X
  }
}
