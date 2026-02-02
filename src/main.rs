use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use std::env;
use tiny_mystery_game::core::{EnvArgsResource, WorldSeed};
use tiny_mystery_game::tiles;
use tiny_mystery_game::village::generation::village;
use tiny_mystery_game::village::generation::village::VillageGenerationPlugin;

fn main() -> AppExit {
    let args: Vec<String> = env::args().collect();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,Village::Generate=debug,Generate::Family=debug,Generate::Villager=debug,Playground::Villager=debug,Playground::Tilemap=debug,Tilemap=debug"
                        .into(),
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
        .add_plugins((TilemapPlugin, VillageGenerationPlugin))
        .add_systems(PreStartup, set_world_seed)
        .add_systems(
            Startup,
            (
                spawn_camera,
                tiles::set_up_tilemap,
                village::request_generate_village,
                // playground.after(village_generation::request_generate_village),
            ),
        )
        .run()
}

// fn playground(
//     families: Query<(), With<village_generation::Family>>,
//     villagers: Query<
//         (),
//         With<tiny_mystery_game::village::model::villager::Villager>,
//     >,
// ) {
//     info!(target: "Village", "families={}, villagers={}", families.iter().len(), villagers.iter().len());
// }

#[derive(Component)]
struct Player {}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera { ..default() },
        Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
        GlobalTransform::default(), // ..default()
    ));
}

fn set_world_seed(mut commands: Commands, env_args: Res<EnvArgsResource>) {
    let args = &env_args.args;

    let seed = if args.len() >= 3 && args[1] == "seed" {
        debug!(target: "PreStartup::SetWorldSeed", "Found seed CLI argument: {}", args[2]);
        &args[2]
    } else {
        debug!(target: "PreStartup::SetWorldSeed", "No seed argument found, using default.");
        "some_seedz"
    };

    info!(target: "PreStartup::SetWorldSeed", "Set World Seed: {}", seed);

    commands.insert_resource(WorldSeed(seed.to_string()));
}
