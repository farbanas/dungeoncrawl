use std::process::exit;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(k) = *key {
        let delta = match k {
            VirtualKeyCode::H => Point::new(-1, 0),
            VirtualKeyCode::L => Point::new(1, 0),
            VirtualKeyCode::J => Point::new(0, 1),
            VirtualKeyCode::K => Point::new(0, -1),
            VirtualKeyCode::Q => exit(0),
            _ => Point::new(0, 0),
        };

        let (player, destination) = <(Entity, &Point)>::query()
            .filter(component::<Player>())
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        let mut did_something = false;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player,
                        destination,
                    },
                ));
            }
        }

        if !did_something {
            if let Ok(mut health) = ecs.entry_mut(player).unwrap().get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + 1);
            }
        }

        *turn_state = TurnState::PlayerTurn;
    };
}
