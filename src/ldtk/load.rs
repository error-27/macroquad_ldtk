use std::{
    fs::File,
    io::{self, BufReader},
    path::{Path, PathBuf},
    result::Result,
};

use macroquad::texture::{load_texture, Texture2D};

use crate::ldtk::parser::*;

use super::types::{LdtkResources, LdtkTileset};

/// Loads an LDtk project from a JSON file.
/// Returns a struct containing the LDtk project resources.
pub async fn load_project(path: &str) -> io::Result<LdtkResources> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json: LdtkJson = serde_json::from_reader(reader)?;

    let path_base = PathBuf::from(path).join("..");

    let mut tilesets: Vec<LdtkTileset> = Vec::new();
    for json_t in &json.defs.tilesets {
        let tex_path = path_base.join(json_t.rel_path.clone().unwrap());
        let tex = load_texture(tex_path.to_str().unwrap()).await;

        if tex.is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, tex.unwrap_err()));
        }

        let tileset = LdtkTileset {
            grid_height: json_t.c_hei,
            grid_width: json_t.c_wid,
            padding: json_t.padding,
            spacing: json_t.spacing,
            tile_grid_size: json_t.tile_grid_size,
            identifier: json_t.identifier.clone(),
            uid: json_t.uid,
            texture: tex.unwrap(),
        };

        tilesets.push(tileset);
    }

    let resources = LdtkResources {
        levels: Vec::new(),
        tilesets: tilesets,
    };

    Ok(resources)
}
