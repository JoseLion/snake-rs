use bevy::{
  asset::AssetServer,
  ecs::{bundle::Bundle, component::Component},
  math::{I8Vec2, Vec2},
  sprite::{Anchor, Sprite},
  transform::components::Transform,
};

use crate::components::grid::Grid;

#[derive(Clone, Copy, PartialEq)]
pub enum FoodVariant {
  Normal,
  Special,
}

#[derive(Component)]
pub struct Food {
  pub position: I8Vec2,
  pub variant: FoodVariant,
}

impl Food {
  pub fn new(variant: FoodVariant, position: I8Vec2) -> Self {
    Food { position, variant }
  }

  pub fn points(&self) -> u32 {
    match self.variant {
      FoodVariant::Normal => 200,
      FoodVariant::Special => 500,
    }
  }

  pub fn texture(&self) -> &str {
    match self.variant {
      FoodVariant::Normal => "food/mouse.png",
      FoodVariant::Special => "food/chameleon.png",
    }
  }
}

#[derive(Bundle)]
pub struct FoodBundle {
  food: Food,
  sprite: Sprite,
  transform: Transform,
}

impl FoodBundle {
  pub fn from(variant: FoodVariant, position: I8Vec2, grid: &Grid, assets: &AssetServer) -> Self {
    let food = Food::new(variant, position);
    let image = assets.load(food.texture());

    FoodBundle {
      food,
      sprite: Sprite {
        anchor: Anchor::TopLeft,
        custom_size: Some(Vec2::splat(grid.cell_size)),
        image,
        ..Sprite::default()
      },
      transform: Transform::from_translation(grid.translation_at(&position)),
    }
  }
}
