pub mod terrain_grid;
pub mod tile_terrain;

use crate::rng::{choose_weighted_value, position_in_range, WeightedValue};
use crate::terrain::terrain_grid::TerrainGrid;
use crate::tiles::tile_index::{GroundTile, Plant, TileIndex};
use bevy::platform::collections::HashMap;
use log::trace;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum TerrainType {
    Empty,
    Forest,
    Meadow,
}

pub const MEADOW_SPRITE_WEIGHTS: [WeightedValue<u32>; 4] = [
    WeightedValue {
        value: GroundTile::GrassFine as u32,
        weight: 3,
    },
    WeightedValue {
        value: GroundTile::GrassFlower as u32,
        weight: 1,
    },
    WeightedValue {
        value: GroundTile::GrassMixed as u32,
        weight: 1,
    },
    WeightedValue {
        value: GroundTile::GrassThick as u32,
        weight: 1,
    },
];

pub type SpriteIndexPicker = fn(hash: u64, grid: &TerrainGrid, x: u32, y: u32) -> u32;

pub fn choose_meadow_sprite(hash: u64, _grid: &TerrainGrid, _x: u32, _y: u32) -> u32 {
    choose_weighted_value(&MEADOW_SPRITE_WEIGHTS, hash)
        .copied()
        .unwrap_or(GroundTile::GrassFine.index())
}

pub fn map_terrain_to_sprite() -> HashMap<TerrainType, Vec<u32>> {
    let mut map = HashMap::<TerrainType, Vec<u32>>::new();
    map.insert(TerrainType::Forest, vec![Plant::Deciduous.index()]);
    map.insert(
        TerrainType::Meadow,
        vec![
            GroundTile::GrassFine.index(),
            GroundTile::GrassFlower.index(),
            GroundTile::GrassMixed.index(),
            GroundTile::GrassThick.index(),
        ],
    );
    map
}

pub fn get_terrain_sprite_index(terrain_type: &TerrainType, hash: &u64) -> u32 {
    trace!(target: "Terrain: Sprites", "Getting terrain sprite index...");
    let terrain_sprite_map = map_terrain_to_sprite();
    let sprite_options = &terrain_sprite_map[terrain_type];
    let position = position_in_range(&(sprite_options.len() as u64), hash);
    trace!(target: "Terrain: Sprites", "Calculated Position {} from hash {}", position, hash);

    sprite_options[position as usize]
}
