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
        let data = vec![fill; (width * height) as usize];
        Self {
            width,
            height,
            data,
        }
    }

    pub fn new_empty(width: u32, height: u32) -> TerrainGrid {
        let data = vec![TileTerrain::default(); (width * height) as usize];
        Self {
            width,
            height,
            data,
        }
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn get(&self, x: u32, y: u32) -> &TileTerrain {
        &self.data[self.index(x, y)]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut TileTerrain {
        let i = self.index(x, y);
        &mut self.data[i]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::TerrainType;

    fn make_terrain(terrain_type: TerrainType, sprite_index: u32) -> TileTerrain {
        TileTerrain {
            terrain_type,
            sprite_index,
        }
    }

    #[test]
    fn test_get_adjacent_corners() {
        let mut grid = TerrainGrid::new_empty(3, 3);
        // North
        *grid.get_mut(1, 0) = make_terrain(TerrainType::Meadow, 1);
        // East
        *grid.get_mut(2, 1) = make_terrain(TerrainType::Forest, 1);
        // South
        *grid.get_mut(1, 2) = make_terrain(TerrainType::Meadow, 2);
        // West
        *grid.get_mut(0, 1) = make_terrain(TerrainType::Meadow, 3);

        let adjacent_to_center = grid.get_adjacent(1, 1);
        assert_eq!(
            adjacent_to_center.north.unwrap().terrain_type,
            TerrainType::Meadow
        );
        assert_eq!(adjacent_to_center.north.unwrap().sprite_index, 1);

        assert_eq!(
            adjacent_to_center.east.unwrap().terrain_type,
            TerrainType::Forest
        );
        assert_eq!(adjacent_to_center.east.unwrap().sprite_index, 1);

        assert_eq!(
            adjacent_to_center.south.unwrap().terrain_type,
            TerrainType::Meadow
        );
        assert_eq!(adjacent_to_center.south.unwrap().sprite_index, 2);

        assert_eq!(
            adjacent_to_center.west.unwrap().terrain_type,
            TerrainType::Meadow
        );
        assert_eq!(adjacent_to_center.west.unwrap().sprite_index, 3);
    }

    #[test]
    fn test_get_adjacent_edges() {
        let grid = TerrainGrid::new_empty(3, 3);

        let top_left_adjacent = grid.get_adjacent(0, 0);
        assert!(top_left_adjacent.north.is_none());
        assert!(top_left_adjacent.west.is_none());
        assert!(top_left_adjacent.east.is_some());
        assert!(top_left_adjacent.south.is_some());

        let top_right_adjacent = grid.get_adjacent(2, 0);
        assert!(top_right_adjacent.north.is_none());
        assert!(top_right_adjacent.west.is_some());
        assert!(top_right_adjacent.east.is_none());
        assert!(top_right_adjacent.south.is_some());

        let bottom_right_adjacent = grid.get_adjacent(2, 2);
        assert!(bottom_right_adjacent.north.is_some());
        assert!(bottom_right_adjacent.west.is_some());
        assert!(bottom_right_adjacent.east.is_none());
        assert!(bottom_right_adjacent.south.is_none());

        let bottom_left_adjacent = grid.get_adjacent(0, 2);
        assert!(bottom_left_adjacent.north.is_some());
        assert!(bottom_left_adjacent.west.is_none());
        assert!(bottom_left_adjacent.east.is_some());
        assert!(bottom_left_adjacent.south.is_none());
    }
}
