use std::time::{SystemTime, UNIX_EPOCH};

use bevy::{
  ecs::{component::Component, error::Result},
  math::{I8Vec2, Vec2, Vec3},
  platform::collections::HashSet,
  sprite::Sprite,
  window::Window,
};
use tinyrand::{RandLim, Seeded, StdRand};

pub struct Bounds {
  pub top: i8,
  pub bottom: i8,
  pub left: i8,
  pub right: i8,
}

impl Bounds {
  pub fn contains(&self, position: &I8Vec2) -> bool {
    position.x >= self.left
      && position.x <= self.right
      && position.y >= self.bottom
      && position.y <= self.top
  }
}

#[derive(Component)]
#[require(Sprite)]
pub struct Grid {
  pub bounds: Bounds,
  pub cell_size: f32,
  pub size: Vec2,
}

impl Grid {
  pub const COLS: u8 = 20;
  pub const ROWS: u8 = 20;

  pub fn from(window: &Window) -> Self {
    let cell_size = window.height() / Self::ROWS as f32;
    let size = Vec2::new(cell_size * Self::COLS as f32, cell_size * Self::ROWS as f32);
    let bounds = Bounds {
      top: Self::ROWS as i8 / 2,
      bottom: (Self::ROWS as i8 / -2) + 1,
      left: Self::COLS as i8 / -2,
      right: (Self::COLS as i8 / 2) - 1,
    };

    Grid {
      bounds,
      cell_size,
      size,
    }
  }

  pub fn translation_at(&self, cell: &I8Vec2) -> Vec3 {
    Vec3::new(
      self.cell_size * cell.x as f32,
      self.cell_size * cell.y as f32,
      0.0,
    )
  }

  pub fn warp(&self, cell: I8Vec2) -> I8Vec2 {
    I8Vec2 {
      x: match cell.x {
        x if x < self.bounds.left => self.bounds.right,
        x if x > self.bounds.right => self.bounds.left,
        _ => cell.x,
      },
      y: match cell.y {
        y if y > self.bounds.top => self.bounds.bottom,
        y if y < self.bounds.bottom => self.bounds.top,
        _ => cell.y,
      },
    }
  }

  pub fn random_position(&self, excludes: &HashSet<I8Vec2>) -> Result<I8Vec2> {
    let now_seed = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let mut rand = StdRand::seed(now_seed);

    loop {
      let row = rand.next_lim(Self::ROWS as u32);
      let col = rand.next_lim(Self::COLS as u32);
      let next = I8Vec2::new(
        col as i8 - (Self::COLS as i8 / 2),
        row as i8 - (Self::ROWS as i8 / 2),
      );

      if self.bounds.contains(&next) && !excludes.contains(&next) {
        break Ok(next);
      }
    }
  }
}
