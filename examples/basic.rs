use macroquad::prelude::*;
use macroquad_ldtk::prelude::*;

#[macroquad::main("Basic LDtk Example")]
async fn main() {
    let main_tileset = load_texture("assets/kenney_platformer.png").await.unwrap();

    let tilesets = [(main_tileset, "kenney_platformer.png")];

    let ldtk_resources = load_project("assets/platformer_example.ldtk", &tilesets)
        .await
        .unwrap();

    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
