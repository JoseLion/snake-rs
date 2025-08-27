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
