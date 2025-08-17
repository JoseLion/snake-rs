use bevy::{
  ecs::component::Component,
  math::{I8Vec2, Vec2, Vec3},
  sprite::Sprite,
  window::Window,
};

#[derive(Component)]
#[require(Sprite)]
pub struct Grid {
  pub cell_size: f32,
  pub size: Vec2,
}

impl Grid {
  pub const COLS: u8 = 20;
  pub const ROWS: u8 = 20;

  pub fn from(window: &Window) -> Self {
    let cell_size = window.height() / Self::ROWS as f32;
    let size = Vec2::new(cell_size * Self::COLS as f32, cell_size * Self::ROWS as f32);

    Self { cell_size, size }
  }

  pub fn translation_at(&self, cell: I8Vec2) -> Vec3 {
    Vec3::new(
      self.cell_size * cell.x as f32,
      self.cell_size * cell.y as f32,
      0.0,
    )
  }

  pub fn warp(&self, cell: I8Vec2) -> I8Vec2 {
    let limit_x = Self::COLS as i8 / 2;
    let limit_y = Self::ROWS as i8 / 2;
    let x = match cell.x {
      x if x < -limit_x => limit_x - 1,
      x if x >= limit_x => -limit_x,
      _ => cell.x,
    };
    let y = match cell.y {
      y if y > limit_y => -limit_y + 1,
      y if y <= -limit_y => limit_y,
      _ => cell.y,
    };

    I8Vec2 { x, y }
  }
}
