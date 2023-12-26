#![warn(clippy::pedantic)]

use bevy::prelude::*;

//Default for the tile sizes.
const TILE_SIZE: u16 = 40;

#[derive(Resource)]
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
        .insert_resource(board)
        .add_systems(Startup, initial_setup)
        .run();
}

fn initial_setup(mut commands: Commands, board: Res<Board>) {
    commands.spawn(Camera2dBundle::default());
    //Draw the grid layout!
    commands
        .spawn(NodeBundle {
            style: Style {
                // Create a grid layout, at 100% of the parent element
                // Height and width.
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![
                    GridTrack::auto(); usize::from(board.squares_wide)
                ],
                grid_template_rows: vec![
                    GridTrack::auto(); usize::from(board.squares_high)
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            //Every other will be black or red!
            for c in 0..board.squares_wide {
                for r in 0..board.squares_high {
                    let color = if (r + c) % 2 == 0 {
                        Color::RED
                    } else {
                        Color::BLACK
                    };
                    builder.spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            ..default()
                        },
                        background_color: BackgroundColor(color),
                        ..default()
                    });
                }
            }
        });
}