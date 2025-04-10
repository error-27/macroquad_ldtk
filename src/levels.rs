//! Module to handle interactions with the levels

use macroquad::prelude::*;

use crate::types::{LdtkEntityInstance, LdtkLayerType, LdtkResources};

impl LdtkResources {
    /// Draws the specified level. The texture array passed in should be the same as when the project was initially loaded.
    /// The `source` rect is in grid coordinates, while the `position` vector is in pixel coordinates.
    pub fn draw_level(
        &self,
        level_coord: (i64, i64),
        textures: &[(Texture2D, &str)],
        position: Vec2,
        source: Option<Rect>,
    ) {
        let lvl = &self
            .levels
            .get(&level_coord)
            .expect(format!("No level at coordinate {:?}", level_coord).as_str()); // I feel a panic is good enough here.
        let tilesets = &self.tilesets;

        for layer in &lvl.layers {
            let layerdef = &self.layer_defs.get(&layer.layerdef_id).unwrap();

            if layerdef.layer_type == LdtkLayerType::Entities {
                continue; // Skip non displayable layers
            }

            if layer.tileset_id.is_none() {
                continue; // This layer has nothing to render
            }
            let tileset_id = layer.tileset_id.as_ref().unwrap();

            let tileset = tilesets.get(tileset_id).unwrap();
            let tex = &textures[tileset.texture_index as usize].0;

            for t in &layer.tiles {
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

    /// Gets all entities in a specified level. Useful for spawning entities on load.
    pub fn get_entities(&self, level_coord: (i64, i64)) -> Vec<&LdtkEntityInstance> {
        let mut entities = Vec::new();

        let level = &self
            .levels
            .get(&level_coord)
            .expect(format!("No level at coordinate {:?}", level_coord).as_str());
        for l in &level.layers {
            for e in &l.entities {
                entities.push(e);
            }
        }

        entities
    }
}
