use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use log::trace;
use std::env;

use tiny_mystery_game::names::{get_first_name, get_surname};
use tiny_mystery_game::rng::{calculate_hash, coin_flip};
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
        .add_systems(Startup, (spawn_tile_sprite, spawn_camera))
        .run()
}

#[derive(Component)]
struct Player {}

#[derive(Resource)]
struct TileSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for TileSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let sprite_size_pixels = 16.0;
        let spritesheet_padding = 1.0;
        let spritesheet_rows = 22;
        let spritesheet_columns = 49;
        let texture_atlas = TextureAtlasLayout::from_grid(
            Vec2::new(sprite_size_pixels, sprite_size_pixels),
            spritesheet_columns,
            spritesheet_rows,
            Some(Vec2::new(spritesheet_padding, spritesheet_padding)),
            None,
        );

        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Self(texture_atlas_handle)
    }
}

fn spawn_tile_sprite(
    mut commands: Commands,
    sprite_atlas: Res<TileSpriteSheet>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let tile_texture: Handle<Image> = asset_server.load("sprites/monochrome-transparent.png");

    commands.spawn(SpriteSheetBundle {
        atlas: TextureAtlas {
            layout: sprite_atlas.0.clone(),
            index: 2,
        },
        texture: tile_texture,
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            color: Color::CRIMSON,
            ..default()
        },
        ..default()
    });
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
