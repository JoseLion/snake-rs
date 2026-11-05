use bevy::{
  asset::AssetServer,
  color::Color,
  ecs::{bundle::Bundle, component::Component},
  text::{FontSmoothing, JustifyText, TextColor, TextFont, TextLayout},
  ui::{
    BackgroundColor, BorderColor, Node, UiRect, Val,
    widget::{Button, Text},
  },
};

use crate::{
  helpers::theme::{FontFace, FontSize, Theme},
  resources::menu::Action,
};

#[derive(Component)]
#[require(Button)]
pub struct MenuButton {
  pub action: Action,
}

#[derive(Bundle)]
pub struct BtnBundle {
  border_color: BorderColor,
  menu_item: MenuButton,
  node: Node,
  background_color: BackgroundColor,
}

impl BtnBundle {
  pub fn new(action: Action) -> Self {
    BtnBundle {
      border_color: BorderColor(BtnBundle::selected_color(false)),
      menu_item: MenuButton { action },
      node: Node {
        border: UiRect::all(Val::Px(3.0)),
        padding: UiRect::axes(Val::Percent(2.0), Val::Percent(1.0)),
        width: Val::Vw(20.0),
        ..Node::default()
      },
      background_color: BackgroundColor(Color::NONE),
    }
  }

  pub fn selected_color(selected: bool) -> Color {
    match selected {
      true => Theme::MENU_SELECTED,
      false => Theme::MENU_IDLE,
    }
  }
}

#[derive(Bundle)]
pub struct BtnTextBundle {
  node: Node,
  text: Text,
  text_font: TextFont,
  text_color: TextColor,
  text_layout: TextLayout,
}

impl BtnTextBundle {
  pub fn new(text: impl Into<String>, assets: &AssetServer) -> Self {
    let font = assets.load(FontFace::PIXELIFY_SANS);

    BtnTextBundle {
      node: Node {
        width: Val::Percent(100.0),
        ..Node::default()
      },
      text: Text::new(text),
      text_font: TextFont::from_font(font)
        .with_font_smoothing(FontSmoothing::None)
        .with_font_size(FontSize::BTN),
      text_color: TextColor::WHITE,
      text_layout: TextLayout {
        justify: JustifyText::Center,
        ..TextLayout::default()
      },
    }
  }
}
