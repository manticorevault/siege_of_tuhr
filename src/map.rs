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
        self.in_bounds(point) && self.tiles[map_index(point.x, point.y)] == TileType::Floor
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
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);

        for y in camera.top_y .. camera.bottom_y {
            for x in camera.left_x .. camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let index = map_index(x, y);

                    match self.tiles[index] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.')
                            );
                        }

                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#')
                            );
                        }
                    }
                }
            }
        }
    }
}