// Terrain entity:
// location, terrain type, secrets, events, descriptions

use bevy::utils::HashMap;

use crate::tiles::tile_index::{GroundTile, Plant};

pub enum TerrainType {
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
