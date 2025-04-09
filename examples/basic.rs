use macroquad::prelude::*;
use macroquad_ldtk::prelude::*;

#[macroquad::main("Basic LDtk Example")]
async fn main() {
    let main_tileset = load_texture("assets/kenney_platformer.png").await.unwrap();
    main_tileset.set_filter(FilterMode::Linear);

    let tilesets = [(main_tileset, "kenney_platformer.png")];

    let res = load_project("assets/platformer_example.ldtk", &tilesets).unwrap();

    let mut current_level = 0;
    let limit = res.levels.len();

    // Load all coins into the level
    let coins: Vec<Coin> = res
        .get_entities(current_level)
        .iter()
        .map(|me| Coin {
            position: me.px_coords,
        })
        .collect();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Space) {
            current_level += 1;
            if current_level == limit {
                current_level = 0;
            }
        }
        res.draw_level(current_level, &tilesets, Vec2::new(0.0, 0.0), None);

        for c in &coins {
            draw_texture_ex(
                &tilesets[0].0,
                c.position[0] as f32,
                c.position[1] as f32,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect {
                        x: 11.0 * 18.0,
                        y: 7.0 * 18.0,
                        w: 18.0,
                        h: 18.0,
                    }),
                    ..Default::default()
                },
            );
        }

        next_frame().await
    }
}

struct Coin {
    position: [i64; 2],
}
