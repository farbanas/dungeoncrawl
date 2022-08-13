use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Name)]
#[read_component(Point)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_point: &Point, #[resource] camera: &Camera) {
    let mut entity_positions = <(Entity, &Point, &Name)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());

    let camera_offset = Point::new(camera.left_x, camera.top_y);
    let corrected_mouse_position = *mouse_point + camera_offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let player_fov = fov.iter(ecs).nth(0).unwrap();

    entity_positions
        .iter(ecs)
        .filter(|(_, pos, _)| {
            **pos == corrected_mouse_position && player_fov.visible_tiles.contains(&pos)
        })
        .for_each(|(entity, _, name)| {
            let mut screen_position = *mouse_point * 4;

            if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                let display = format!("{}: {} hp", &name.0, health.current);

                let text_width = if display.len() % 2 == 1 {
                    display.len() + 3
                } else {
                    display.len() + 4
                };

                screen_position.x -= (display.len() as i32) / 2;
                screen_position.y -= 2;
                let text_position = Point::new(screen_position.x + 2, screen_position.y - 1);

                draw_batch.draw_double_box(
                    Rect::with_size(
                        screen_position.x - 1,
                        screen_position.y - 1,
                        text_width as i32 + 1,
                        2,
                    ),
                    ColorPair::new(WHITE, WHITE),
                );
                draw_batch.bar_horizontal(
                    screen_position,
                    text_width,
                    health.current,
                    health.max,
                    ColorPair::new(RED, BLACK),
                );
                draw_batch.print(text_position, &display);
            } else {
                let display = name.0.clone();
                let text_width = display.len() as i32 + 3;
                let text_position = Point::new(screen_position.x + 2, screen_position.y + 1);
                draw_batch.draw_double_box(
                    Rect::with_size(screen_position.x, screen_position.y, text_width, 2),
                    ColorPair::new(WHITE, WHITE),
                );
                draw_batch.print(text_position, &display);
            };
        });

    draw_batch.submit(10100).expect("Draw error");
}
