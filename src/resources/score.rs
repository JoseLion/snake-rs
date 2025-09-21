use bevy::ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct Score {
  pub value: u32,
}

impl Score {
  pub fn reset(&mut self) {
    *self = Score::default();
  }
}
