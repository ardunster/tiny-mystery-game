mod tile_index;

use crate::tiles::tile_index::{
    Building, Cobble, Critter, Face, Fence, GroundTile, Path, Person, Plant, Water,
};
use bevy::color::palettes::tailwind::GREEN_700;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::*;

pub const SPRITE_SIZE_PIXELS: u32 = 16;

#[derive(Resource)]
pub struct TileSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for TileSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let spritesheet_padding = 1;
        let spritesheet_rows = 22;
        let spritesheet_columns = 49;
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(SPRITE_SIZE_PIXELS, SPRITE_SIZE_PIXELS),
            spritesheet_columns,
            spritesheet_rows,
            Some(UVec2::new(spritesheet_padding, spritesheet_padding)),
            None,
        );

        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Self(texture_atlas_handle)
    }
}

pub fn spawn_tile_sprite(
    mut commands: Commands,
    sprite_atlas: Res<TileSpriteSheet>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let tile_texture: Handle<Image> = asset_server.load("sprites/monochrome-transparent.png");

    commands.spawn((
        SpriteBundle {
            texture: tile_texture,
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                color: Color::from(GREEN_700),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: sprite_atlas.0.clone(),
            index: GroundTile::GrassFine as usize,
            ..default()
        },
    ));
}

pub fn set_up_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let tilemap_entity = commands.spawn_empty().id();
    let map_size = TilemapSize { x: 32, y: 18 };
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(GroundTile::GrassFine as u32),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize {
        x: SPRITE_SIZE_PIXELS as f32,
        y: SPRITE_SIZE_PIXELS as f32,
    };

    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    let tile_texture: Handle<Image> = asset_server.load("sprites/monochrome_packed.png");

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_texture),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..default()
    });
}
