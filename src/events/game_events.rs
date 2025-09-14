use bevy::ecs::event::Event;

use crate::components::food::FoodVariant;

#[derive(Event)]
pub struct EatFoodEvent {
  pub variant: FoodVariant,
}

impl EatFoodEvent {
  pub fn new(variant: FoodVariant) -> Self {
    EatFoodEvent { variant }
  }
}
