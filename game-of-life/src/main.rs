#![warn(clippy::pedantic)]

use bevy::prelude::*;

//Default for the tile sizes.
const TILE_SIZE: u16 = 40;

struct Board {
    squares_wide: u16,
    squares_high: u16,
}

fn main() {
    println!("Bevy app starting!");
    let cols = 20;
    let rows = 20;
    let board = Board {squares_wide: cols, squares_high: rows};
    let window_width = TILE_SIZE * board.squares_wide;
    let window_height =  TILE_SIZE * board.squares_high;
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Conway's Game of Life".into(),
                    resolution: (f32::from(window_width), f32::from(window_height)).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, initial_setup)
        .run();
}

fn initial_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    //Draw the background!
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::BLUE.into(),
        ..default()
    });
}