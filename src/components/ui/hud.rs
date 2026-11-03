use bevy::{
  ecs::component::Component,
  ui::{Node, widget::Text},
};

#[derive(Component)]
#[require(Node)]
pub struct Hud;

#[derive(Component)]
#[require(Node)]
pub struct LeftPane;

#[derive(Component)]
#[require(Node)]
pub struct RightPane;

#[derive(Component)]
#[require(Text)]
pub struct ScoreText;

#[derive(Component)]
#[require(Text)]
pub struct SpeedText;

#[derive(Component)]
#[require(Node)]
pub struct BonusHeading;

#[derive(Component)]
#[require(Text)]
pub struct BonusTimeText;
