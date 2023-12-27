#![warn(clippy::pedantic)]

use bevy::prelude::*;

//Default for the tile sizes.
const TILE_SIZE: u16 = 40;
const UPDATE_RATE_SEC: f64 = 0.5;

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
        .insert_resource(Time::<Fixed>::from_seconds(UPDATE_RATE_SEC))
        .add_systems(FixedUpdate, update_board)
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

fn update_board(mut query: Query<(&mut BackgroundColor, &GridLocation)>, mut board: ResMut<Board>) {
    //Fetch the neighbor counts.
    let neighbor_counts = get_alive_neighbor_counts(board.as_ref());
    for (mut color, grid_loc) in &mut query {
        let c = usize::from(grid_loc.column);
        let r = usize::from(grid_loc.row);
        let cur = board.squares[c][r];
        let n = neighbor_counts[c][r];
        let mut new_state = cur;
        if cur {
            // Live cell
            //fewer than two live neighbours dies, as if by underpopulation.
            if n < 2 {
                //Underpop
                new_state = false;
            }
            //two or three live neighbours lives on to the next generation.
            if n == 2 || n == 3 {
                //We live!
                new_state = true;
            }
            //more than three live neighbours dies, as if by overpopulation.
            if n > 3 {
                //Overpop
                new_state = false;
            }
        } else {
            // Dead Cell
            //exactly three live neighbours becomes a live cell, as if by reproduction.
            if n == 3 {
                //breeeed
                new_state = true;
            }
        }
        //Update the data
        board.squares[c][r] = new_state;
        if new_state {
            *color = Color::BLACK.into();
        } else {
            *color = Color::WHITE.into();
        }
    }
}

fn get_alive_neighbor_counts(board: &Board) -> Vec<Vec<usize>> {
    let height = usize::from(board.squares_high);
    let width = usize::from(board.squares_wide);
    let mut neighbor_counts = vec![vec![0; height]; width];
    for (c, row) in neighbor_counts.iter_mut().enumerate() {
        for (r, item) in  row.iter_mut().enumerate() {
            let mut neighbors = 0;
            //Top
            if r > 0 {
                //T/L
                if c > 0 && board.squares[c-1][r-1] {
                    neighbors += 1;
                }
                //T/C
                if board.squares[c][r-1] {
                    neighbors += 1;
                }
                //T/R
                if c+1 < width && board.squares[c+1][r-1] {
                    neighbors += 1;
                }
            }
            //Left
            if c > 0 && board.squares[c-1][r] {
                neighbors += 1;
            }
            //Right
            if c+1 < width && board.squares[c+1][r] {
                neighbors += 1;
            }
            //Bottom
            if r+1 < height {
                //B/L
                if c > 0 && board.squares[c-1][r+1] {
                    neighbors += 1;
                }
                //B/C
                if board.squares[c][r+1] {
                    neighbors += 1;
                }
                //B/R
                if c+1 < width && board.squares[c+1][r+1] {
                    neighbors += 1;
                }
            }
            *item = neighbors;
        }
    }
    neighbor_counts
}