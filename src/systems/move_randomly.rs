use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MoveRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn move_randomly(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movables = <(Entity, &Point)>::query().filter(component::<MoveRandomly>());
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut rng = RandomNumberGenerator::new();
    movables.iter(ecs).for_each(|(entity, pos)| {
        let shift = match rng.range(0, 4) {
            0 => Point::new(1, 0),
            1 => Point::new(-1, 0),
            2 => Point::new(0, 1),
            _ => Point::new(0, -1),
        };
        let destination = shift + *pos;
        let mut attacked = false;
        positions
            .iter(ecs)
            .filter(|(_, target_pos, _)| **target_pos == destination)
            .for_each(|(victim, _, _)| {
                if ecs
                    .entry_ref(*victim)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: *entity,
                            victim: *victim,
                        },
                    ));
                }
                attacked = true;
            });

        if !attacked {
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        }
    });
}
