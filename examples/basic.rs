use macroquad::prelude::*;
use macroquad_ldtk::prelude::*;

#[macroquad::main("Basic LDtk Example")]
async fn main() {
    let main_tileset = load_texture("assets/kenney_platformer.png").await.unwrap();
    main_tileset.set_filter(FilterMode::Linear);

    let tilesets = [(main_tileset, "kenney_platformer.png")];

    let ldtk_resources = load_project("assets/platformer_example.ldtk", &tilesets).unwrap();

    let mut current_level = 0;
    let limit = ldtk_resources.levels.len();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Space) {
            current_level += 1;
            if current_level == limit {
                current_level = 0;
            }
        }
        draw_level(
            current_level,
            &ldtk_resources,
            &tilesets,
            Vec2::new(0.0, 0.0),
            None,
        );

        next_frame().await
    }
}
