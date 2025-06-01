use crate::terrain::tile_terrain::TileTerrain;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct TerrainGrid {
    width: u32,
    height: u32,
    data: Vec<TileTerrain>,
}

pub struct AdjacentTiles<'a> {
    north: Option<&'a TileTerrain>,
    east: Option<&'a TileTerrain>,
    south: Option<&'a TileTerrain>,
    west: Option<&'a TileTerrain>,
}

impl TerrainGrid {
    pub fn new(width: u32, height: u32, fill: TileTerrain) -> TerrainGrid {
        let data = vec![fill; (width * height).into()];
        Self {
            width,
            height,
            data,
        }
    }

    pub fn new_empty(width: u32, height: u32) -> TerrainGrid {
        let data = vec![TileTerrain::default(); (width * height).into()];
        Self {
            width,
            height,
            data,
        }
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x).into()
    }

    pub fn get(&self, x: u32, y: u32) -> &TileTerrain {
        &self.data[self.index(x, y)]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut TileTerrain {
        &mut self.data[self.index(x, y)]
    }

    pub fn get_adjacent(&self, x: u32, y: u32) -> AdjacentTiles {
        AdjacentTiles {
            north: if y > 0 {
                Some(self.get(x, y - 1))
            } else {
                None
            },
            east: if x + 1 < self.width {
                Some(self.get(x + y, y))
            } else {
                None
            },
            south: if y + 1 < self.height {
                Some(self.get(x, y + 1))
            } else {
                None
            },
            west: if x > 0 {
                Some(self.get(x - 1, y))
            } else {
                None
            },
        }
    }
}
