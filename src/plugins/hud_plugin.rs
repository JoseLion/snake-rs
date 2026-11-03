use bevy::{
  app::{App, Plugin, Update},
  asset::AssetServer,
  ecs::{
    change_detection::DetectChanges,
    entity::Entity,
    query::{Or, With},
    schedule::IntoScheduleConfigs,
    system::{Commands, Query, Res, Single},
  },
  state::condition::in_state,
  text::{FontSmoothing, TextColor, TextFont},
  time::{Fixed, Time},
  ui::{
    FlexDirection, Node, UiRect, Val,
    widget::{ImageNode, Text},
  },
};

use crate::{
  components::ui::hud::{BonusHeading, BonusTimeText, RightPane, ScoreText, SpeedText},
  helpers::theme::{FontFace, FontSize},
  plugins::game_plugin::InGameSystemSet,
  resources::{bonus::Bonus, level_up::LevelUp, score::Score},
  states::game_state::GameState,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (
        update_score,
        update_speed,
        on_bonus_active,
        update_bonus_time,
      )
        .run_if(in_state(GameState::InGame))
        .after(InGameSystemSet),
    );
  }
}

fn update_score(score_res: Res<Score>, mut text_query: Query<&mut Text, With<ScoreText>>) {
  if score_res.is_changed() {
    let score = score_res.into_inner();

    text_query.iter_mut().for_each(|mut text| {
      text.0 = format!("{}", score.value);
    });
  }
}

fn update_speed(time_res: Res<Time<Fixed>>, mut text_query: Query<&mut Text, With<SpeedText>>) {
  if time_res.is_changed() {
    let time = time_res.into_inner();
    let speed = LevelUp::INIT_SPEED / time.timestep().as_secs_f64();

    text_query.iter_mut().for_each(|mut text| {
      text.0 = format!("x{:.2}", speed);
    });
  }
}

fn on_bonus_active(
  bonus_res: Res<Bonus>,
  assets_res: Res<AssetServer>,
  pane_query: Single<Entity, With<RightPane>>,
  bonus_heading_query: Query<Entity, With<BonusHeading>>,
  mut commands: Commands,
) {
  if bonus_res.is_changed() {
    let bonus = bonus_res.into_inner();

    if bonus.active && bonus_heading_query.is_empty() {
      let pane_id = pane_query.into_inner();
      let assets = assets_res.into_inner();
      let font = assets.load(FontFace::PRESS_START_2P);

      commands.entity(pane_id).with_children(|pane| {
        pane
          .spawn((
            BonusHeading,
            Node {
              column_gap: Val::Px(12.0),
              flex_direction: FlexDirection::Row,
              margin: UiRect::top(Val::Percent(5.0)),
              width: Val::Percent(100.0),
              ..Node::default()
            },
          ))
          .with_children(|heading| {
            heading.spawn((
              Node {
                width: Val::Px(28.0),
                height: Val::Px(28.0),
                ..Node::default()
              },
              ImageNode {
                image: assets.load("icons/chameleon.png"),
                ..ImageNode::default()
              },
            ));

            heading.spawn((
              Text::new("Bonus"),
              TextColor::WHITE,
              TextFont::from_font_size(FontSize::H2)
                .with_font(font.clone())
                .with_font_smoothing(FontSmoothing::None),
            ));
          });

        pane.spawn((
          BonusTimeText,
          Text::new(format!("{:.2}", bonus.timer.remaining().as_secs_f32())),
          TextColor::WHITE,
          TextFont::from_font_size(FontSize::H3)
            .with_font(font.clone())
            .with_font_smoothing(FontSmoothing::None),
        ));
      });
    }
  }
}

#[allow(clippy::type_complexity)]
fn update_bonus_time(
  bonus_res: Res<Bonus>,
  bonus_ui_query: Query<Entity, Or<(With<BonusHeading>, With<BonusTimeText>)>>,
  mut text_query: Query<&mut Text, With<BonusTimeText>>,
  mut commands: Commands,
) {
  if bonus_res.is_changed() {
    let bonus = bonus_res.into_inner();

    match bonus.active {
      true => {
        text_query.iter_mut().for_each(|mut text| {
          text.0 = format!("{:.2}", bonus.timer.remaining().as_secs_f32());
        });
      }
      false => {
        bonus_ui_query.iter().for_each(|entity| {
          commands.entity(entity).despawn();
        });
      }
    }
  }
}
