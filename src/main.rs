use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::env;
use tiny_mystery_game::names::{get_first_name, get_surname};
use tiny_mystery_game::rng::{calculate_hash, coin_flip};
use tiny_mystery_game::tiles;
use tiny_mystery_game::tiles::TileSpriteSheet;
use tiny_mystery_game::villagers::Gender;

fn main() -> AppExit {
    let args: Vec<String> = env::args().collect();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,Playground=debug".into(),
                    level: bevy::log::Level::DEBUG,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(EnvArgsResource { args })
        .init_resource::<TileSpriteSheet>()
        .add_systems(
            Startup,
            (playground, tiles::spawn_tile_sprite, spawn_camera),
        )
        .run()
}

fn playground(env_args: Res<EnvArgsResource>) {
    let args = &env_args.args;

    let stringy_seed = if args.len() >= 3 && args[1] == "seed" {
        debug!(target: "Playground", "Found seed CLI argument: {}", args[2]);
        &args[2]
    } else {
        debug!(target: "Playground", "No seed argument found, using default.");
        "some_seedz"
    };

    debug!(target: "Playground", "Resulting seed: {}", stringy_seed);

    for position in 0..3 {
        let seed_with_pos = stringy_seed.to_owned() + &position.to_string();

        debug!(target: "Playground", "seed with position: {}", seed_with_pos);
        let hash = calculate_hash(&seed_with_pos);

        let gender = match coin_flip(&hash) {
            true => Gender::Male,
            false => Gender::Female,
        };

        debug!(target: "Playground",
            "Got a name: {} {}",
            get_first_name(&hash, &gender),
            get_surname(&hash)
        );
    }
}

#[derive(Resource)]
struct EnvArgsResource {
    args: Vec<String>,
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
