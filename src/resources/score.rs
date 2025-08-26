use bevy::ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct Score {
  pub value: u32,
}
