use bevy::{
  ecs::resource::Resource,
  time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct Bonus {
  pub active: bool,
  pub progress: u32,
  pub threshold: u32,
  pub timer: Timer,
}

impl Default for Bonus {
  fn default() -> Self {
    Bonus {
      active: false,
      progress: 0,
      threshold: 1400,
      timer: Timer::from_seconds(6.0, TimerMode::Once),
    }
  }
}

impl Bonus {
  pub fn activate(&mut self) {
    self.active = true;
    self.progress = 0;
  }

  pub fn deactivate(&mut self) {
    self.active = false;
    self.timer.reset();
    self.progress = 0;
  }
}
