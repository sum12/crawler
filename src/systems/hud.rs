use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn hud(ecs: &SubWorld) {
    let mut health = <&Health>::query().filter(component::<Player>());
    let player_health = health.iter(ecs).nth(0).unwrap();

    let mut db = DrawBatch::new();
    db.target(2);
    db.print_centered(1, "Explore the Dungeon, Cursor keys to move");
    db.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    db.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );
    db.submit(10000).expect("Batch Hud Error");
}
