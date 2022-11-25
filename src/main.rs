use bracket_lib::prelude::*;
mod map;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

use prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(35, 1, "The Siege of Tuhr");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("The Siege of Tuhr")
        .build()?;

        main_loop(context, State{})
}

