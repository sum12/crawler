use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let dest = *pos
            + match rng.range(0, 4) {
                0 => Point::new(0, -1),
                1 => Point::new(-1, 0),
                2 => Point::new(1, 0),
                _ => Point::new(0, 1),
            };
        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination: dest,
            },
        ));
    })
}
