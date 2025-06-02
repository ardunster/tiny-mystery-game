pub mod terrain_grid;
pub mod tile_terrain;

use crate::rng::{position_in_range, WeightedValue};
use crate::tiles::tile_index::{GroundTile, Plant};
use bevy::platform::collections::HashMap;
use bevy::prelude::Component;
use log::trace;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum TerrainType {
    Empty,
    Forest,
    Meadow,
}

pub const MEADOW_SPRITE_WEIGHTS: [WeightedValue<u32>; 4] = [
    WeightedValue {
        value: GroundTile::GrassFine.into(),
        weight: 3,
    },
    WeightedValue {
        value: GroundTile::GrassFlower.into(),
        weight: 1,
    },
    WeightedValue {
        value: GroundTile::GrassMixed.into(),
        weight: 1,
    },
    WeightedValue {
        value: GroundTile::GrassThick.into(),
        weight: 1,
    },
];

pub fn map_terrain_to_sprite() -> HashMap<TerrainType, Vec<u32>> {
    let mut map = HashMap::<TerrainType, Vec<u32>>::new();
    map.insert(TerrainType::Forest, vec![Plant::Deciduous.into()]);
    map.insert(
        TerrainType::Meadow,
        vec![
            GroundTile::GrassFine.into(),
            GroundTile::GrassFlower.into(),
            GroundTile::GrassMixed.into(),
            GroundTile::GrassThick.into(),
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
