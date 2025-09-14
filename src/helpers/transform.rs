use bevy::math::Quat;

pub struct Flip {
  pub x: bool,
  pub y: bool,
}

pub struct SpriteTx {
  pub rotation: Quat,
  pub flip: Flip,
}
