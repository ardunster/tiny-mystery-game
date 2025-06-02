use crate::terrain::TerrainType;
use crate::tiles::tile_index::{GroundTile, TileIndex};
use bevy::prelude::Component;

// Terrain entity:
// location, terrain type, secrets, events, descriptions

#[derive(Component, Clone)]
pub struct TileTerrain {
    pub terrain_type: TerrainType,
    pub sprite_index: u32,
}

impl Default for TileTerrain {
    fn default() -> Self {
        Self {
            terrain_type: TerrainType::Empty,
            sprite_index: GroundTile::Empty.index(),
        }
    }
}
