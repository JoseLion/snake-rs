use bevy::{
  color::Color,
  ecs::{bundle::Bundle, component::Component},
  ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val},
};

#[derive(Component)]
#[require(Node)]
pub struct Overlay;

#[derive(Bundle)]
pub struct OverlayBundle {
  pub background_color: BackgroundColor,
  pub node: Node,
  pub overlay: Overlay,
}

impl OverlayBundle {
  pub fn new(bg_color: impl Into<Option<Color>>) -> Self {
    let color = bg_color.into().unwrap_or(Color::NONE);

    OverlayBundle {
      background_color: BackgroundColor(color),
      node: Node {
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        height: Val::Vh(100.0),
        justify_content: JustifyContent::Center,
        row_gap: Val::Px(16.0),
        width: Val::Vw(100.0),
        ..Node::default()
      },
      overlay: Overlay,
    }
  }
}
