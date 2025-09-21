use bevy::{
  app::{App, AppExit, Plugin, Update},
  ecs::{
    event::EventWriter,
    hierarchy::Children,
    query::With,
    schedule::{Condition, IntoScheduleConfigs},
    system::{Commands, Query, Res, ResMut},
  },
  input::{ButtonInput, keyboard::KeyCode},
  state::{
    condition::in_state,
    state::{NextState, OnExit},
  },
  text::TextColor,
  ui::{BorderColor, widget::Text},
};

use crate::{
  components::ui::menu_button::{BtnBundle, MenuButton},
  resources::menu::{Action, Menu},
  states::game_state::GameState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
  fn build(&self, app: &mut App) {
    let in_menu_state = in_state(GameState::MainMenu).or(in_state(GameState::Paused));

    app
      .add_systems(
        Update,
        (navigation, show_selected, on_select).run_if(in_menu_state),
      )
      .add_systems(OnExit(GameState::MainMenu), despawn_menu)
      .add_systems(OnExit(GameState::Paused), despawn_menu);
  }
}

fn navigation(keyboard: Res<ButtonInput<KeyCode>>, mut menu: ResMut<Menu>) {
  keyboard.into_inner().get_just_pressed().for_each(|key| {
    match key {
      KeyCode::ArrowUp => menu.prev(),
      KeyCode::ArrowDown => menu.next(),
      _ => (),
    };
  });
}

fn show_selected(
  menu_res: Res<Menu>,
  mut menu_items: Query<(&MenuButton, &mut BorderColor, &Children)>,
  mut text_query: Query<&mut TextColor, With<Text>>,
) {
  let menu = menu_res.into_inner();

  menu_items
    .iter_mut()
    .for_each(|(button, mut border_color, children)| {
      let is_selected = menu.is_selected(button.action);

      border_color.0 = BtnBundle::selected_color(is_selected);

      children.iter().for_each(|child_id| {
        if let Ok(mut text_color) = text_query.get_mut(*child_id) {
          text_color.0 = BtnBundle::selected_color(is_selected);
        }
      });
    });
}

fn on_select(
  keyboard: Res<ButtonInput<KeyCode>>,
  menu_res: Res<Menu>,
  mut next_state: ResMut<NextState<GameState>>,
  mut event_writer: EventWriter<AppExit>,
) {
  if keyboard.into_inner().just_pressed(KeyCode::Enter) {
    let menu = menu_res.into_inner();

    match menu.selected() {
      Action::Start => next_state.set(GameState::InGame),
      Action::Resume => next_state.set(GameState::InGame),
      Action::Quit => {
        event_writer.write(AppExit::Success);
      }
    };
  }
}

fn despawn_menu(menu: Res<Menu>, mut commands: Commands) {
  let id = menu.into_inner().id();

  commands.entity(id).despawn();
  commands.remove_resource::<Menu>();
}
