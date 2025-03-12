use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use macroquad::texture::Texture2D;

use crate::ldtk::parser::*;

use super::types::{LdtkResources, LdtkTileset};

/// Loads an LDtk project from a JSON file.
/// Returns a struct containing the LDtk project resources.
pub async fn load_project(path: &str, textures: &[(Texture2D, &str)]) -> io::Result<LdtkResources> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json: LdtkJson = serde_json::from_reader(reader)?;

    let mut path_base = PathBuf::from(path);
    path_base.pop();

    // Load tilesets
    let mut tilesets: Vec<LdtkTileset> = Vec::new();
    for json_t in &json.defs.tilesets {
        let tex_i = textures
            .iter()
            .position(|(_, name)| *name == json_t.rel_path.as_ref().unwrap().as_str())
            .unwrap();

        let tileset = LdtkTileset {
            grid_height: json_t.c_hei,
            grid_width: json_t.c_wid,
            padding: json_t.padding,
            spacing: json_t.spacing,
            tile_grid_size: json_t.tile_grid_size,
            identifier: json_t.identifier.clone(),
            uid: json_t.uid,
            texture_index: tex_i as u32,
        };

        tilesets.push(tileset);
    }

    let resources = LdtkResources {
        levels: Vec::new(),
        tilesets: tilesets,
    };

    Ok(resources)
}
