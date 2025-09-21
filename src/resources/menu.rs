use std::sync::Arc;

use bevy::ecs::{entity::Entity, resource::Resource};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
  Start,
  Resume,
  Quit,
}

impl Action {
  pub fn label(&self) -> &str {
    match self {
      Action::Start => "Start Game",
      Action::Resume => "Resume Game",
      Action::Quit => "Quit Game",
    }
  }
}

#[derive(Resource)]
pub struct Menu {
  id: Entity,
  actions: Arc<[Action]>,
  selected: Action,
}

impl Menu {
  const MAIN: [Action; 2] = [Action::Start, Action::Quit];
  const PAUSE: [Action; 2] = [Action::Resume, Action::Quit];

  pub fn main_menu_from(id: Entity) -> Self {
    Menu {
      id,
      actions: Arc::new(Self::MAIN),
      selected: Action::Start,
    }
  }

  pub fn pause_menu_from(id: Entity) -> Self {
    Menu {
      id,
      actions: Arc::new(Self::PAUSE),
      selected: Action::Resume,
    }
  }

  pub fn id(&self) -> Entity {
    self.id
  }

  pub fn selected(&self) -> Action {
    self.selected
  }

  pub fn actions(&self) -> &[Action] {
    &self.actions
  }

  pub fn next(&mut self) {
    if let Some(idx) = self.selected_idx() {
      let next_idx = (idx + 1) % self.actions.len();
      self.selected = self.actions[next_idx];
    }
  }

  pub fn prev(&mut self) {
    if let Some(idx) = self.selected_idx() {
      let size = self.actions.len();
      let prev_idx = (size + idx - 1) % size;
      self.selected = self.actions[prev_idx];
    }
  }

  pub fn is_selected(&self, action: Action) -> bool {
    self.selected == action
  }

  fn selected_idx(&self) -> Option<usize> {
    self.actions.iter().position(|&a| self.is_selected(a))
  }
}
