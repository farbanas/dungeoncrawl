use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, position: Point) {
    ecs.push((
        Player,
        position,
        Render {
            color: ColorPair::new(RGBA::from_u8(255, 255, 255, 255), RGBA::from_u8(0, 0, 0, 0)),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, position: Point) {
    let (health, name, glyph) = match rng.range(0, 20) {
        0 => ettin(),
        1 | 2 | 3 => ogre(),
        4 | 5 | 6 | 7 | 8 | 9 => orc(),
        _ => goblin(),
    };

    ecs.push((
        Enemy,
        ChasingPlayer,
        position,
        Render {
            color: ColorPair::new(RGBA::from_u8(255, 255, 255, 255), RGBA::from_u8(0, 0, 0, 0)),
            glyph,
        },
        Health {
            current: health,
            max: health,
        },
        Name(name),
        FieldOfView::new(6),
    ));
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(RGBA::from_u8(255, 255, 255, 255), RGBA::from_u8(0, 0, 0, 0)),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}

pub fn goblin() -> (i32, String, FontCharType) {
    (1, String::from("Goblin"), to_cp437('g'))
}

pub fn orc() -> (i32, String, FontCharType) {
    (2, String::from("Orc"), to_cp437('o'))
}

pub fn ogre() -> (i32, String, FontCharType) {
    (4, String::from("Ogre"), to_cp437('O'))
}

pub fn ettin() -> (i32, String, FontCharType) {
    (6, String::from("Ettin"), to_cp437('E'))
}

/*
pub fn dragon() -> (i32, String, FontCharType) {
    (10, String::from("Dragon"), to_cp437('D'))
}
*/
