use std::collections::HashMap;

use macroquad::prelude::*;

use crate::types::{LdtkLayerInstance, LdtkResources, LdtkTileset};

pub fn draw_level(level_idx: usize, res: &LdtkResources, textures: &[(Texture2D, &str)]) {
    let lvl = &res.levels[level_idx];

    for layer in &lvl.layers {
        draw_layer(layer, &res.tilesets, &textures);
    }
}

fn draw_layer(
    layer: &LdtkLayerInstance,
    tilesets: &HashMap<String, LdtkTileset>,
    textures: &[(Texture2D, &str)],
) {
    let tileset = tilesets.get(&layer.tileset_id).unwrap();
    let tex = &textures[tileset.texture_index as usize].0;

    for t in &layer.grid_tiles {
        draw_texture_ex(
            tex,
            t.px_coords[0] as f32,
            t.px_coords[1] as f32,
            WHITE,
            DrawTextureParams {
                source: Some(Rect {
                    x: t.src_coords[0] as f32,
                    y: t.src_coords[1] as f32,
                    w: tileset.tile_grid_size as f32,
                    h: tileset.tile_grid_size as f32,
                }),
                ..Default::default()
            },
        );
    }
}
