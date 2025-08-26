use bevy::ecs::{entity::Entity, event::Event};

use crate::components::food::FoodVariant;

#[derive(Event)]
pub struct EatFoodEvent {
  pub id: Entity,
  pub variant: FoodVariant,
}

impl EatFoodEvent {
  pub fn new(id: Entity, variant: FoodVariant) -> Self {
    EatFoodEvent { id, variant }
  }
}
