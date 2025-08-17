use bevy::{ecs::component::Component, math::I8Vec2, sprite::Sprite};

#[derive(Component)]
#[require(Sprite)]
pub struct Segment {
  pub position: I8Vec2,
}

impl Segment {
  pub fn new(x: i8, y: i8) -> Self {
    Self {
      position: I8Vec2::new(x, y),
    }
  }
}
