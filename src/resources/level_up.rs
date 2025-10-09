use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct LevelUp {
  pub progress: u32,
  pub threshold: u32,
}

impl Default for LevelUp {
  fn default() -> Self {
    LevelUp {
      progress: 0,
      threshold: 1000,
    }
  }
}

impl LevelUp {
  pub const INIT_SPEED: f64 = 0.25;

  pub fn reset(&mut self) {
    *self = LevelUp::default();
  }
}
