use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(RandomMovement)]

pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut movers = <(&mut Point, &RandomMovement)>::query();

    movers 
        .iter_mut(ecs)
        .for_each(|(pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;

            if map.walkable_path(destination) {
                *pos = destination;
            }
        }
    );
}