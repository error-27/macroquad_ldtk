use macroquad::prelude::*;

use crate::types::LdtkResources;

pub fn render_level(res: &LdtkResources, textures: &[(Texture2D, &str)]) {
    let tileset_id = res
        .levels
        .first()
        .unwrap()
        .layers
        .first()
        .unwrap()
        .tileset_id
        .clone();

    let tileset_ref = res.tilesets.get(&tileset_id).expect("tilset hashmap broke");
    let tex_index = tileset_ref.texture_index;

    draw_texture(&textures[tex_index as usize].0, 0.0, 0.0, WHITE);
}
