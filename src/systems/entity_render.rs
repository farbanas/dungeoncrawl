use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    let camera_offset = Point::new(camera.left_x, camera.top_y);
    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(position, render)| {
            draw_batch.set(*position - camera_offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}
