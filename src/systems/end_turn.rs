use crate::prelude::*;

#[system]
#[read_component(AmuletOfYala)]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
pub fn end_turn(ecs: &SubWorld, #[resource] current_turn: &mut TurnState) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());

    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();

    let mut next_turn = match current_turn {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => *current_turn,
    };

    player_hp.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            next_turn = TurnState::GameOver;
        }
        if pos == amulet_pos {
            next_turn = TurnState::Victory;
        }
    });

    *current_turn = next_turn;
}
