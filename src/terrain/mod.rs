// Terrain entity:
// location, terrain type, secrets, events, descriptions

use bevy::prelude::Component;
use bevy::utils::HashMap;

use crate::tiles::tile_index::{GroundTile, Plant};

#[derive(Eq, Hash, PartialEq)]
pub enum TerrainType {
    Empty,
    Forest,
    Meadow,
}

pub fn map_terrain_to_sprite() -> HashMap<TerrainType, Vec<u32>> {
    let mut map = HashMap::<TerrainType, Vec<u32>>::new();
    map.insert(TerrainType::Forest, vec![Plant::Deciduous as u32]);
    map.insert(
        TerrainType::Meadow,
        vec![
            GroundTile::GrassFine as u32,
            GroundTile::GrassFlower as u32,
            GroundTile::GrassMixed as u32,
            GroundTile::GrassThick as u32,
        ],
    );
    map
}

#[derive(Component)]
pub struct TileTerrain {
    pub terrain_type: TerrainType,
    pub sprite_index: u32,
}

impl Default for TileTerrain {
    fn default() -> Self {
        Self {
            terrain_type: TerrainType::Empty,
            sprite_index: GroundTile::Empty as u32,
        }
    }
}
