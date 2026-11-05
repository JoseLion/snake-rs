use std::iter::once;

use bevy::{
  app::{App, Plugin, Startup},
  asset::{AssetServer, Assets},
  core_pipeline::core_2d::Camera2d,
  ecs::{
    entity::Entity,
    error::Result,
    query::With,
    schedule::IntoScheduleConfigs,
    system::{Commands, Query, Res, ResMut},
  },
  image::{TextureAtlas, TextureAtlasLayout},
  math::{I8Vec2, UVec2, Vec2},
  platform::collections::HashSet,
  render::view::Visibility,
  sprite::Sprite,
  state::state::{OnEnter, OnTransition},
  text::{FontSmoothing, JustifyText, LineBreak, TextColor, TextFont, TextLayout},
  transform::components::Transform,
  ui::{AlignItems, FlexDirection, Node, UiRect, Val, ZIndex, widget::Text},
  window::{PrimaryWindow, Window},
};

use crate::{
  components::{
    food::{FoodBundle, FoodVariant},
    grid::Grid,
    segment::{Segment, SegmentBundle, SegmentKind},
    snake::Snake,
    ui::{
      hud::{Hud, LeftPane, RightPane, ScoreText, SpeedText},
      menu_button::{BtnBundle, BtnTextBundle},
      overlay::OverlayBundle,
    },
  },
  helpers::theme::{FontFace, FontSize, Theme},
  resources::{bonus::Bonus, level_up::LevelUp, menu::Menu, score::Score},
  states::game_state::GameState,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<Score>()
      .init_resource::<LevelUp>()
      .init_resource::<Bonus>()
      .add_systems(Startup, setup_camera)
      .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
      .add_systems(
        OnTransition {
          exited: GameState::MainMenu,
          entered: GameState::InGame,
        },
        (
          setup_grid,
          (setup_snake, setup_segments),
          setup_food,
          setup_hud,
        )
          .chain(),
      )
      .add_systems(OnEnter(GameState::Paused), setup_pause_menu)
      .add_systems(OnEnter(GameState::GameOver), setup_gameover);
  }
}

fn setup_camera(mut commands: Commands) {
  commands.spawn(Camera2d);
}

fn setup_main_menu(assets_res: Res<AssetServer>, mut commands: Commands) {
  let mut overlay = commands.spawn(OverlayBundle::new(None));
  let menu = Menu::main_menu_from(overlay.id());
  let assets = assets_res.into_inner();

  overlay.with_children(|container| {
    menu.actions().iter().for_each(|action| {
      container
        .spawn(BtnBundle::new(*action))
        .with_child(BtnTextBundle::new(action.label(), assets));
    });
  });

  commands.insert_resource(menu);
}

fn setup_pause_menu(assets_res: Res<AssetServer>, mut commands: Commands) {
  let mut overlay = commands.spawn(OverlayBundle::new(Theme::OVERLAY));
  let menu = Menu::pause_menu_from(overlay.id());
  let assets = assets_res.into_inner();

  overlay.with_children(|container| {
    menu.actions().iter().for_each(|action| {
      container
        .spawn(BtnBundle::new(*action))
        .with_child(BtnTextBundle::new(action.label(), assets));
    });
  });

  commands.insert_resource(menu);
}

fn setup_grid(window_query: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) -> Result {
  let window = window_query.single()?;
  let grid = Grid::from(window);
  let sprite = Sprite {
    color: Theme::WORLD_COLOR,
    custom_size: Some(grid.size),
    ..Sprite::default()
  };

  commands.spawn((grid, sprite));

  Ok(())
}

fn setup_snake(
  grid_query: Query<(Entity, &Grid)>,
  assets_res: Res<AssetServer>,
  mut layout_asset: ResMut<Assets<TextureAtlasLayout>>,
  mut commands: Commands,
) -> Result {
  let (grid_id, grid) = grid_query.single()?;
  let assets = assets_res.into_inner();
  let image = assets.load("snake/head.png");
  let tile_size = UVec2::splat(32);
  let padding = Some(UVec2::splat(8));
  let texture_layout = TextureAtlasLayout::from_grid(tile_size, 2, 1, padding, None);
  let layout = layout_asset.add(texture_layout);
  let snake = Snake::default();
  let position = snake.position;
  let sprite_tx = snake.sprite_tx();

  commands.entity(grid_id).with_child((
    snake,
    Sprite {
      custom_size: Some(Vec2::splat(grid.cell_size)),
      image,
      texture_atlas: Some(TextureAtlas { layout, index: 0 }),
      flip_x: sprite_tx.flip.x,
      flip_y: sprite_tx.flip.y,
      ..Sprite::default()
    },
    Transform::from_translation(grid.translation_at(&position)).with_rotation(sprite_tx.rotation),
  ));

  Ok(())
}

fn setup_segments(
  grid_query: Query<(Entity, &Grid)>,
  assets_res: Res<AssetServer>,
  mut layout_asset: ResMut<Assets<TextureAtlasLayout>>,
  mut commands: Commands,
) -> Result {
  let assets = assets_res.into_inner();
  let (grid_id, grid) = grid_query.single()?;

  commands.entity(grid_id).with_children(|parent| {
    (1..=4).for_each(|i| {
      let segment = Segment {
        direction: I8Vec2::Y,
        kind: match i {
          4 => SegmentKind::Tail,
          _ => SegmentKind::Body,
        },
        position: I8Vec2::new(0, -i),
        has_food: false,
      };

      parent.spawn(SegmentBundle::new(
        segment,
        grid,
        assets,
        &mut layout_asset,
        I8Vec2::Y,
      ));
    });
  });

  Ok(())
}

fn setup_food(
  grid_query: Query<(Entity, &Grid)>,
  snake_query: Query<&Snake>,
  segment_query: Query<&Segment>,
  assets_res: Res<AssetServer>,
  mut commands: Commands,
) -> Result {
  let assets = assets_res.into_inner();
  let (grid_id, grid) = grid_query.single()?;
  let snake = snake_query.single()?;
  let excludes = once(snake.position)
    .chain(segment_query.iter().map(|s| s.position))
    .collect::<HashSet<_>>();
  let position = grid.random_position(&excludes);

  commands.entity(grid_id).with_child(FoodBundle::from(
    FoodVariant::Normal,
    position,
    grid,
    assets,
  ));

  Ok(())
}

fn setup_gameover(assets: Res<AssetServer>, mut commands: Commands) {
  let font = assets.into_inner().load(FontFace::PIXELIFY_SANS);

  commands
    .spawn(OverlayBundle::new(Theme::OVERLAY))
    .with_children(|overlay| {
      overlay.spawn((
        Text::new("Game Over"),
        TextColor::WHITE,
        TextFont::from_font(font.clone())
          .with_font_smoothing(FontSmoothing::None)
          .with_font_size(40.0),
        TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
      ));

      overlay.spawn((
        Text::new("Press \"Enter\" to continue"),
        TextColor::WHITE,
        TextFont::from_font(font.clone())
          .with_font_smoothing(FontSmoothing::None)
          .with_font_size(25.0),
        TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
      ));
    });
}

fn setup_hud(assets: Res<AssetServer>, mut commands: Commands) -> Result {
  let font = assets.into_inner().load(FontFace::PRESS_START_2P);

  commands
    .spawn((
      Hud,
      Node {
        height: Val::Vh(100.0),
        width: Val::Vw(100.0),
        ..Node::default()
      },
      ZIndex(-10),
    ))
    .with_children(|parent| {
      parent
        .spawn((
          LeftPane,
          Node {
            flex_direction: FlexDirection::Column,
            flex_grow: 1.0,
            height: Val::Vh(100.0),
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Percent(1.0)),
            ..Node::default()
          },
        ))
        .with_children(|pane| {
          pane.spawn((
            Text::new("Score"),
            TextColor::WHITE,
            TextFont::from_font(font.clone())
              .with_font_smoothing(FontSmoothing::None)
              .with_font_size(FontSize::H1),
            TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
          ));

          pane.spawn((
            ScoreText,
            Text::new("0"),
            TextColor::WHITE,
            TextFont::from_font(font.clone())
              .with_font_smoothing(FontSmoothing::None)
              .with_font_size(FontSize::H2),
            TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
          ));
        });

      parent.spawn((
        Node {
          height: Val::Vh(100.0),
          width: Val::Vh(100.0),
          ..Node::default()
        },
        Visibility::Hidden,
      ));

      parent
        .spawn((
          RightPane,
          Node {
            flex_direction: FlexDirection::Column,
            flex_grow: 1.0,
            height: Val::Vh(100.0),
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Percent(1.0)),
            ..Node::default()
          },
        ))
        .with_children(|pane| {
          pane.spawn((
            Text::new("Speed"),
            TextColor::WHITE,
            TextFont::from_font(font.clone())
              .with_font_smoothing(FontSmoothing::None)
              .with_font_size(FontSize::H1),
            TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
          ));

          pane.spawn((
            SpeedText,
            Text::new("x1.00"),
            TextColor::WHITE,
            TextFont::from_font(font.clone())
              .with_font_smoothing(FontSmoothing::None)
              .with_font_size(FontSize::H2),
            TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
          ));
        });
    });

  Ok(())
}
