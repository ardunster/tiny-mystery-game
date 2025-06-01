use crate::terrain::tile_terrain::TileTerrain;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct TerrainGrid {
    width: u32,
    height: u32,
    data: Vec<TileTerrain>,
}
