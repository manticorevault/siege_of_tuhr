use crate::prelude::*;

pub struct Player {
    pub position: Point
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self {
            position
        }
    }

    // A function to render the character
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@')
        );
    }

    // A function to make the character movable
    // on top of the floor
    pub fn update(&mut self, ctx: &mut BTerm, map : &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero()
            };

            // Defines the player's new position
            let new_position = self.position + delta;

            if map.walkable_path(new_position) {
                self.position = new_position;
            }
        }
    }
}