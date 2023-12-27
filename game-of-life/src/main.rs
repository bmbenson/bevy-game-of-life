#![warn(clippy::pedantic)]

use bevy::prelude::*;

//Default for the tile sizes.
const TILE_SIZE: u16 = 40;

#[derive(Resource)]
struct Board {
    squares_wide: u16,
    squares_high: u16,
    squares: Vec<Vec<bool>>,
}

#[derive(Component, Debug)]
struct GridLocation {
    row: u16,
    column: u16
}

fn main() {
    println!("Bevy app starting!");
    let cols = 20;
    let rows = 20;
    // Create a 2d vector where every other square is on or off.
    // This is equivalent to a nested for loop over cols then row elements.
    let board_state = (0..cols).map(|col| 
        (0..rows).map(|row| 
            (col + row) % 2 == 0)
            .collect())
    .collect();
    let board = Board {squares_wide: cols, squares_high: rows, squares: board_state};
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
        .add_systems(Update, button_system)
        .run();
}

fn initial_setup(mut commands: Commands, board: Res<Board>) {
    commands.spawn(Camera2dBundle::default());
    //Button style
    let button_style = Style {
        display: Display::Grid,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
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
            //Every other will be black or white!
            for c in 0..board.squares_wide {
                for r in 0..board.squares_high {
                    let color = if board.squares[usize::from(c)][usize::from(r)] {
                        Color::BLACK
                    } else {
                        Color::WHITE
                    };
                    let grid_loc = GridLocation {column: c, row: r};
                    builder.spawn(
                        (ButtonBundle {
                            style: button_style.clone(),
                            background_color: BackgroundColor(color),
                            ..default()
                        }, grid_loc)
                    );
                }
            }
        });
}

#[allow(clippy::type_complexity)]
fn button_system(mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &GridLocation
    ),
    (Changed<Interaction>, With<Button>),
>, mut board: ResMut<Board>) {
    for (interaction, mut color, grid_loc) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let r = usize::from(grid_loc.row);
                let c = usize::from(grid_loc.column);
                //Get the game state.
                let cur = board.squares[c][r];
                println!("Button pressed at ({c},{r}) -- Currently:{cur}");

                if cur { //Alive to dead
                    *color = Color::WHITE.into();
                }
                else {
                    *color = Color::BLACK.into();
                }
                board.squares[c][r] = !cur;
            },
            Interaction::Hovered | Interaction::None => {},
        }
    }
}