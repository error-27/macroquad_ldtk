use macroquad::prelude::*;
use macroquad_ldtk::prelude::*;

#[macroquad::main("Gridvania Example")]
async fn main() {
    let main_tileset = load_texture("assets/kenney_platformer.png").await.unwrap();
    main_tileset.set_filter(FilterMode::Nearest);

    let tilesets = [(main_tileset, "kenney_platformer.png")];

    let ldtk_res = load_project("assets/gridvania_example.ldtk", &tilesets).unwrap();
    loop {
        clear_background(BLUE);
        ldtk_res.draw_level((0, 0), &tilesets, Vec2::new(0.0, 0.0), None);

        next_frame().await;
    }
}
