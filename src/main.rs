use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::TilemapPlugin;
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
                    filter: "info,Playground::Seed=debug,Tilemap=debug".into(),
                    level: bevy::log::Level::DEBUG,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Tiny Mystery Game"),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(EnvArgsResource { args })
        // .init_resource::<TileSpriteSheet>()
        .add_plugins(TilemapPlugin)
        .add_systems(
            Startup,
            // (playground, tiles::spawn_tile_sprite, spawn_camera),
            (spawn_camera, tiles::set_up_tilemap),
        )
        .run()
}

fn playground(env_args: Res<EnvArgsResource>) {
    let args = &env_args.args;

    let stringy_seed = if args.len() >= 3 && args[1] == "seed" {
        debug!(target: "Playground::Seed", "Found seed CLI argument: {}", args[2]);
        &args[2]
    } else {
        debug!(target: "Playground::Seed", "No seed argument found, using default.");
        "some_seedz"
    };

    debug!(target: "Playground::Seed", "Resulting seed: {}", stringy_seed);

    for position in 0..3 {
        let seed_with_pos = stringy_seed.to_owned() + &position.to_string();

        debug!(target: "Playground::Villager", "seed with position: {}", seed_with_pos);
        let hash = calculate_hash(&seed_with_pos);

        let gender = match coin_flip(&hash) {
            true => Gender::Male,
            false => Gender::Female,
        };

        debug!(target: "Playground::Villager",
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

#[derive(Resource)]
struct WorldSeed(String);

// TODO: How to implement an init for seed from resource?!

#[derive(Component)]
struct Player {}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
        ..default()
    });
}
