mod tile_index;

use crate::tiles::tile_index::{
    Building, Cobble, Critter, Face, Fence, GroundTile, Path, Person, Plant, Water,
};
use bevy::color::palettes::tailwind::GREEN_700;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource)]
pub struct TileSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for TileSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let sprite_size_pixels = 16;
        let spritesheet_padding = 1;
        let spritesheet_rows = 22;
        let spritesheet_columns = 49;
        let texture_atlas = TextureAtlasLayout::from_grid(
            UVec2::new(sprite_size_pixels, sprite_size_pixels),
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
