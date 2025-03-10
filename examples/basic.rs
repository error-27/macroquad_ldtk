use macroquad::prelude::*;
use macroquad_ldtk::prelude::*;

#[macroquad::main("Basic LDtk Example")]
async fn main() {
    let ldtk_resources = load_project("assets/platformer_example.ldtk")
        .await
        .unwrap();

    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
