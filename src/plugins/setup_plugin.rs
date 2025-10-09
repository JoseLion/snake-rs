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
  sprite::Sprite,
  state::state::{OnEnter, OnTransition},
  text::{FontSmoothing, JustifyText, LineBreak, TextColor, TextFont, TextLayout},
  transform::components::Transform,
  ui::widget::Text,
  window::{PrimaryWindow, Window},
};

use crate::{
  components::{
    food::{FoodBundle, FoodVariant},
    grid::Grid,
    segment::{Segment, SegmentBundle, SegmentKind},
    snake::Snake,
    ui::{
      menu_button::{BtnBundle, BtnTextBundle},
      overlay::OverlayBundle,
    },
  },
  helpers::theme::Theme,
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
      .add_systems(
        OnEnter(GameState::MainMenu),
        (setup_main_menu, global_font).chain(),
      )
      .add_systems(
        OnTransition {
          exited: GameState::MainMenu,
          entered: GameState::InGame,
        },
        (setup_grid, (setup_snake, setup_segments), setup_food).chain(),
      )
      .add_systems(
        OnEnter(GameState::Paused),
        (setup_pause_menu, global_font).chain(),
      )
      .add_systems(
        OnEnter(GameState::GameOver),
        (setup_gameover, global_font).chain(),
      );
  }
}

fn setup_camera(mut commands: Commands) {
  commands.spawn(Camera2d);
}

fn setup_main_menu(mut commands: Commands) {
  let mut overlay = commands.spawn(OverlayBundle::new(None));
  let menu = Menu::main_menu_from(overlay.id());

  overlay.with_children(|container| {
    menu.actions().iter().for_each(|action| {
      container
        .spawn(BtnBundle::new(*action))
        .with_child(BtnTextBundle::new(action.label()));
    });
  });

  commands.insert_resource(menu);
}

fn setup_pause_menu(mut commands: Commands) {
  let mut overlay = commands.spawn(OverlayBundle::new(Theme::OVERLAY));
  let menu = Menu::pause_menu_from(overlay.id());

  overlay.with_children(|container| {
    menu.actions().iter().for_each(|action| {
      container
        .spawn(BtnBundle::new(*action))
        .with_child(BtnTextBundle::new(action.label()));
    });
  });

  commands.insert_resource(menu);
}

fn global_font(assets_res: Res<AssetServer>, mut text_query: Query<&mut TextFont>) {
  let assets = assets_res.into_inner();
  let font = assets.load("fonts/PixelifySans-Variable.ttf");

  text_query.iter_mut().for_each(|mut text| {
    text.font = font.clone();
    text.font_smoothing = FontSmoothing::None;
  });
}

fn setup_grid(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) -> Result {
  let window = window_query.single()?;
  let grid = Grid::from(window);
  let sprite = Sprite {
    color: Theme::WORLD_COLOR,
    custom_size: Some(grid.size),
    ..Default::default()
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

fn setup_gameover(mut commands: Commands) {
  commands
    .spawn(OverlayBundle::new(Theme::OVERLAY))
    .with_children(|overlay| {
      overlay.spawn((
        Text::new("Game Over"),
        TextColor::WHITE,
        TextFont {
          font_size: 40.0,
          ..TextFont::default()
        },
        TextLayout {
          justify: JustifyText::Center,
          linebreak: LineBreak::NoWrap,
        },
      ));

      overlay.spawn((
        Text::new("Press \"Enter\" to continue"),
        TextColor::WHITE,
        TextFont {
          font_size: 25.0,
          ..TextFont::default()
        },
        TextLayout {
          justify: JustifyText::Center,
          linebreak: LineBreak::NoWrap,
        },
      ));
    });
}
