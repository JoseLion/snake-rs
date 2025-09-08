use bevy::{
  ecs::resource::Resource,
  time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct MouthTimer {
  pub value: Timer,
}

impl Default for MouthTimer {
  fn default() -> Self {
    MouthTimer {
      value: Timer::from_seconds(0.35, TimerMode::Once),
    }
  }
}
