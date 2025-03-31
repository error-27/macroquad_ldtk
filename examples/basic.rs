use macroquad::prelude::*;
use macroquad_ldtk::prelude::*;

#[macroquad::main("Basic LDtk Example")]
async fn main() {
    let main_tileset = load_texture("assets/kenney_platformer.png").await.unwrap();
    main_tileset.set_filter(FilterMode::Linear);

    let tilesets = [(main_tileset, "kenney_platformer.png")];

    let ldtk_resources = load_project("assets/platformer_example.ldtk", &tilesets)
        .await
        .unwrap();

    loop {
        clear_background(BLACK);

        draw_level(0, &ldtk_resources, &tilesets, Vec2::new(0.0, 0.0));

        next_frame().await
    }
}
