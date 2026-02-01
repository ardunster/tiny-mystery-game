pub mod terrain_grid;
pub mod tile_terrain;

use crate::rng::weighted_value::{choose_weighted_value, WeightedValue};
use crate::terrain::terrain_grid::{TerrainGrid, TerrainGridSize};
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

pub const FOREST_SPRITE_WEIGHTS: [WeightedValue<u32>; 1] = [WeightedValue {
    value: Plant::Deciduous as u32,
    weight: 3,
}];

pub type SpriteIndexPicker =
    fn(hash: u64, grid: &TerrainGrid, x: u32, y: u32) -> u32;

pub fn pick_meadow_sprite_index(
    hash: u64,
    _grid: &TerrainGrid,
    _x: u32,
    _y: u32,
) -> u32 {
    choose_weighted_value(&MEADOW_SPRITE_WEIGHTS, hash)
        .copied()
        .unwrap_or(GroundTile::GrassFine.index())
}

pub fn pick_forest_sprite_index(
    hash: u64,
    _grid: &TerrainGrid,
    _x: u32,
    _y: u32,
) -> u32 {
    choose_weighted_value(&FOREST_SPRITE_WEIGHTS, hash)
        .copied()
        .unwrap_or(Plant::Deciduous.index())
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

pub fn map_terrain_to_sprite_index_picker(
) -> HashMap<TerrainType, SpriteIndexPicker> {
    let mut map = HashMap::<TerrainType, SpriteIndexPicker>::new();
    map.insert(TerrainType::Meadow, pick_meadow_sprite_index);
    map.insert(TerrainType::Forest, pick_forest_sprite_index);
    map
}

pub fn get_terrain_sprite_index(
    hash: &u64,
    terrain_type: &TerrainType,
    terrain_grid: &TerrainGrid,
    terrain_grid_size: &TerrainGridSize,
) -> u32 {
    trace!(target: "Terrain: Sprites", "Getting terrain sprite index...");
    let terrain_sprite_picker_map = map_terrain_to_sprite_index_picker();
    let sprite_index_picker = &terrain_sprite_picker_map[terrain_type];
    let sprite_index = sprite_index_picker(
        *hash,
        terrain_grid,
        terrain_grid_size.x,
        terrain_grid_size.y,
    );
    trace!(
        target: "Terrain: Sprites", "Calculated Sprite Index {} from hash {}",
        sprite_index,
        hash);

    sprite_index
}
