#![warn(clippy::pedantic)]

use bevy::prelude::*;

// Window starting dimensions
const WINDOW_START_HEIGHT: f32 = 800.0;
const WINDOW_START_WIDTH: f32 = 700.0;
fn main() {
    println!("Bevy app starting!");
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Conway's Game of Life".into(),
                    resolution: (WINDOW_START_WIDTH, WINDOW_START_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .run();
}