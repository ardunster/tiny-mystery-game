use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use log::trace;
use std::env;

use tiny_mystery_game::names::{get_first_name, get_surname};
use tiny_mystery_game::rng::{calculate_hash, coin_flip};
use tiny_mystery_game::tiles;
use tiny_mystery_game::tiles::TileSpriteSheet;
use tiny_mystery_game::villagers::Gender;

fn main() {
    // Use RUST_LOG=trace to see env_logger output.
    // env_logger::init();
    let args: Vec<String> = env::args().collect();

    let stringy_seed = if args.len() >= 3 && args[1] == "seed" {
        &args[2]
    } else {
        "some_seedz"
    };

    for position in 0..3 {
        let seed_with_pos = stringy_seed.to_owned() + &position.to_string();

        trace!(target: "Main", "seed with position: {}", seed_with_pos);
        let hash = calculate_hash(&seed_with_pos);

        let gender = match coin_flip(&hash) {
            true => Gender::Male,
            false => Gender::Female,
        };

        trace!(target: "Main",
            "Got a name: {} {}",
            get_first_name(&hash, &gender),
            get_surname(&hash)
        );
    }

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<TileSpriteSheet>()
        .add_systems(Startup, (tiles::spawn_tile_sprite, spawn_camera))
        .run()
}

#[derive(Component)]
struct Player {}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
