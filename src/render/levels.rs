use std::collections::HashMap;

use macroquad::prelude::*;

use crate::types::{LdtkLayerInstance, LdtkResources, LdtkTileset};

/// Draws the specified level. The texture array passed in should be the same as when the project was initially loaded.
pub fn draw_level(
    level_idx: usize,
    res: &LdtkResources,
    textures: &[(Texture2D, &str)],
    position: Vec2,
    source: Option<Rect>,
) {
    let lvl = &res.levels[level_idx];
    let tilesets = &res.tilesets;

    for layer in &lvl.layers {
        let tileset = tilesets.get(&layer.tileset_id).unwrap();
        let tex = &textures[tileset.texture_index as usize].0;

        for t in &layer.grid_tiles {
            if let Some(s) = source {
                let grid_x = t.px_coords[0] as f32 / tileset.tile_grid_size as f32;
                let grid_y = t.px_coords[1] as f32 / tileset.tile_grid_size as f32;

                // Don't render outside of the specified source rectangle
                if grid_x < s.x || grid_x >= s.x + s.w || grid_y < s.y || grid_y >= s.y + s.h {
                    continue;
                }
            }
            draw_texture_ex(
                tex,
                t.px_coords[0] as f32 + position.x,
                t.px_coords[1] as f32 + position.y,
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
}
