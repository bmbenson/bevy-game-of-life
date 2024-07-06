#![warn(clippy::pedantic)]

use bevy::prelude::*;

//Default for the tile sizes.
const TILE_SIZE: u16 = 8;
const STATUS_BAR_PX: f32 = 40.0;
const UPDATE_RATE_SEC: f64 = 0.5;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Running,
    Paused,
}

#[derive(Resource)]
struct Board {
    squares_wide: u16,
    squares_high: u16,
    squares: Vec<Vec<bool>>,
    alive_squares: usize,
}

#[derive(Resource, Default)]
struct GameMetadata {
    iterations: usize
}

#[derive(Component)]
struct IterationText;

#[derive(Component)]
struct GameStateText;

#[derive(Component, Debug)]
struct GridLocation {
    row: u16,
    column: u16
}

#[derive(Event, Default)]
struct BoardNeedsUpdateEvent;

#[derive(Event, Default)]
struct BoardNeedsDrawingEvent;

#[derive(Event, Default)]
struct StatusBarNeedsDrawingEvent;

fn main() {
    println!("Bevy app starting!");
    let cols = 100;
    let rows = 100;
    // Create a 2d vector where every other square is on or off.
    // This is equivalent to a nested for loop over cols then row elements.
    let board_state = (0..cols).map(|col| 
        (0..rows).map(|row| 
            (col + row) % 2 == 0)
            .collect())
    .collect();
    let board = Board {squares_wide: cols, squares_high: rows, squares: board_state, alive_squares: usize::from(cols) * usize::from(rows) / 2};
    let game_metadata = GameMetadata::default();
    let window_width = f32::from(TILE_SIZE * board.squares_wide);
    let window_height =  f32::from(TILE_SIZE * board.squares_high) + STATUS_BAR_PX;
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Conway's Game of Life".into(),
                    resolution: (window_width, window_height).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .insert_resource(board)
        .insert_resource(game_metadata)
        .insert_resource(Time::<Fixed>::from_seconds(UPDATE_RATE_SEC))
        .add_event::<BoardNeedsUpdateEvent>()
        .add_event::<BoardNeedsDrawingEvent>()
        .add_event::<StatusBarNeedsDrawingEvent>()
        .init_state::<GameState>()
        .add_systems(FixedUpdate, game_tick_timer.run_if(in_state(GameState::Running)))
        .add_systems(Startup, initial_setup)
        .add_systems(Update, (button_system, keyboard_system, update_board, draw_board, status_bar_text_update).chain())
        .run();
}

#[allow(clippy::needless_pass_by_value)]
fn initial_setup(mut commands: Commands, board: Res<Board>, metadata: ResMut<GameMetadata>) {
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
                //Create a grid layout,
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![
                    GridTrack::auto()
                ],
                //Top Row will take up all the space after the bottom row is complete.
                grid_template_rows: vec![
                    GridTrack::flex(1.0), GridTrack::px(STATUS_BAR_PX)
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            //Game Area
            builder.spawn(NodeBundle {
                style: Style {
                    //Create a grid layout,
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
            .with_children(|game_area_builder| {
                //Every other will be black or white!
                for c in 0..board.squares_wide {
                    for r in 0..board.squares_high {
                        //Set the color based on the board state.
                        let color = if board.squares[usize::from(c)][usize::from(r)] {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        };
                        let grid_loc = GridLocation {column: c, row: r};
                        game_area_builder.spawn(
                            (ButtonBundle {
                                style: button_style.clone(),
                                background_color: BackgroundColor(color),
                                ..default()
                            }, grid_loc)
                        );
                    }
                }
            });
            //Status Tray
            builder.spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    padding: UiRect::all(Val::Px(6.0)),
                    grid_template_rows: vec![
                        GridTrack::auto()
                    ],
                    //Left slot, right slot.
                    grid_template_columns: vec![
                        GridTrack::auto(), GridTrack::auto()
                    ],
                    ..default()
                },
                ..default()
            })
            .with_children(|tray_builder| {
                tray_builder.spawn((TextBundle::from_section(
                    "Running: [space] to pause, [c] to clear.",
                    TextStyle {
                        font: Handle::default(),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                ), GameStateText));
                tray_builder.spawn((TextBundle::from_section(
                    format!("Iter:{}; Alive:{}", metadata.iterations, board.alive_squares),
                    TextStyle {
                        font: Handle::default(),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                ).with_text_justify(JustifyText::Right), IterationText));
            });
        });
}

fn game_tick_timer(mut game_board_update_needed: EventWriter<BoardNeedsUpdateEvent>) {
    game_board_update_needed.send_default();
}

#[allow(clippy::type_complexity)]
fn button_system(mut interaction_query: Query<
    (
        &Interaction,
        &GridLocation,
    ),
    (Changed<Interaction>, With<Button>),
>, mut board: ResMut<Board>, mut board_needs_drawing: EventWriter<BoardNeedsDrawingEvent>,
    mut status_bar_needs_update: EventWriter<StatusBarNeedsDrawingEvent>) {
    for (interaction, grid_loc) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let r = usize::from(grid_loc.row);
                let c = usize::from(grid_loc.column);
                //Get the game state.
                let cur = board.squares[c][r];
                if cur {
                    board.alive_squares -= 1;
                } else {
                    board.alive_squares += 1;
                }
                println!("Button pressed at ({c},{r}) -- Currently:{cur}");
                board.squares[c][r] = !cur;
                board_needs_drawing.send_default();
                status_bar_needs_update.send_default();
            },
            Interaction::Hovered | Interaction::None => {},
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn keyboard_system(keyboard_input: Res<ButtonInput<KeyCode>>, game_state: Res<State<GameState>>, mut next_game_state: ResMut<NextState<GameState>>, 
    mut board: ResMut<Board>, mut board_needs_drawing_events: EventWriter<BoardNeedsDrawingEvent>,
    mut board_update_events: EventWriter<BoardNeedsUpdateEvent>, mut status_bar_needs_redraw: EventWriter<StatusBarNeedsDrawingEvent>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match game_state.to_owned() {
            GameState::Running => {
                println!("Pausing");
                next_game_state.set(GameState::Paused);
            },
            GameState::Paused => {
                println!("Running");
                next_game_state.set(GameState::Running);
            },
        }
        status_bar_needs_redraw.send_default();
    }
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        println!("Clear");
        for c in 0..usize::from(board.squares_wide) {
            for r in 0..usize::from(board.squares_high) {
                board.squares[c][r] = false;
            }
        }
        board.alive_squares = 0;
        board_needs_drawing_events.send_default();
        status_bar_needs_redraw.send_default();
    }
    if keyboard_input.just_pressed(KeyCode::KeyN) {
        println!("Next");
        //Send an update to update the board state, including the iterations.
        if game_state.to_owned() == GameState::Paused {
            board_update_events.send_default();
        } else {
            println!("Next disabled when not paused.");
        } 
    }
}

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
fn status_bar_text_update(mut text_params: ParamSet<(Query<&mut Text, With<GameStateText>>, Query<&mut Text, With<IterationText>>)>, board: Res<Board>,
    metadata: Res<GameMetadata>, game_state: Res<State<GameState>>, mut status_bar_needs_redraw: EventReader<StatusBarNeedsDrawingEvent>) {
    if status_bar_needs_redraw.is_empty() {
        return;
    }
    status_bar_needs_redraw.clear();

    let mut game_state_query = text_params.p0();
    match game_state.to_owned() {
        GameState::Running => {
            game_state_query.single_mut().sections[0].value = "Running: [space] to pause, [c] to clear.".to_string();
        },
        GameState::Paused => {
            game_state_query.single_mut().sections[0].value = "Paused: [space] to resume, [c] to clear, [n] for next.".to_string();
        },
    }
    let mut iter_state_query = text_params.p1();
    let new_text = format!("Iter:{}; Alive:{}", metadata.iterations, board.alive_squares);
    iter_state_query.single_mut().sections[0].value = new_text;
}



fn update_board(mut query: Query<&GridLocation>, mut board: ResMut<Board>, mut metadata: ResMut<GameMetadata>,
    mut board_update_events: EventReader<BoardNeedsUpdateEvent>, mut board_needs_draw_event: EventWriter<BoardNeedsDrawingEvent>,
    mut status_bar_needs_update: EventWriter<StatusBarNeedsDrawingEvent>) {
    //Fetch the neighbor counts.
    if board_update_events.is_empty() {
        return;
    }
    board_update_events.clear();
    let neighbor_counts = get_alive_neighbor_counts(board.as_ref());
    let mut alive_count = 0;
    for grid_loc in &mut query {
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
        if new_state {
            alive_count += 1;
        }
        //Update the data
        board.squares[c][r] = new_state;
    }
    board.alive_squares = alive_count;
    metadata.iterations += 1;
    board_needs_draw_event.send_default();
    status_bar_needs_update.send_default();
}

#[allow(clippy::needless_pass_by_value)]
fn draw_board(mut query: Query<(&mut BackgroundColor, &GridLocation)>, board: Res<Board>, mut board_needs_draw_events: EventReader<BoardNeedsDrawingEvent>) {
    if board_needs_draw_events.is_empty() {
        return;
    }
    board_needs_draw_events.clear();
    for (mut color, grid_loc) in &mut query {
        let alive = board.squares[usize::from(grid_loc.column)][usize::from(grid_loc.row)];
        if alive {
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