use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    // A function to determine boundaries 
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH
                && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // A function to determine if the player's move is valid,
    // either inside boundaries and not on a Wall Tile
    pub fn walkable_path(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_index(point.x, point.y)]==TileType::Floor
    }

    // A function to indicate an error if the coordinates are,
    // outside the map boundaries.
    pub fn test_index(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None 
        } else {
            Some(map_index(point.x, point.y))
        }
    }

    // A function to render the map
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = map_index(x, y);

                match self.tiles[index] {
                    TileType::Wall => {
                        ctx.set(x, y, GRAY, BLACK,
                            to_cp437('#'));
                    }

                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK,
                            to_cp437('.')
                        )
                    }
                }
            }
        }
    }
}