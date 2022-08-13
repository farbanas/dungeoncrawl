use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn hud(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .find_map(|health| (Some((health.current, health.max))));

    draw_batch.print_centered(1, "Explore the Dungeon. hljk keys to move.");

    if let Some((current_health, max_health)) = health {
        draw_batch.bar_horizontal(
            Point::zero(),
            SCREEN_WIDTH * 2,
            current_health,
            max_health,
            ColorPair::new(RED, BLACK),
        );

        draw_batch.print_color_centered(
            0,
            format!("Health: {}/{}", current_health, max_health),
            ColorPair::new(WHITE, RED),
        );
    };

    draw_batch.submit(10000).expect("Drawing error");
}
