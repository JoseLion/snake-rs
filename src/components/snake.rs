use bevy::{ecs::component::Component, math::I8Vec2, sprite::Sprite};

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
