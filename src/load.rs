use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use macroquad::texture::Texture2D;

use crate::{
    parser::*,
    types::{LdtkLayerDef, LdtkLayerType},
};

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
    let mut tilesets: HashMap<String, LdtkTileset> = HashMap::new();
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

        tilesets.insert(textures[tex_i].1.to_owned(), tileset);
    }

    // Load layer definitions
    let mut layerdefs: HashMap<String, LdtkLayerDef> = HashMap::new();
    for json_l in &json.defs.layers {
        let layer_type = match json_l.layer_definition_type.as_str() {
            "IntGrid" => LdtkLayerType::IntGrid,
            "Tiles" => LdtkLayerType::Tiles,
            "AutoLayer" => LdtkLayerType::AutoLayer,
            "Entities" => LdtkLayerType::Entities,
            _ => panic!("Invalid LDtk file loaded!"),
        };

        let layerdef = LdtkLayerDef {
            layer_type: layer_type,
            identifier: json_l.identifier.clone(),
            opacity: json_l.display_opacity,
            grid_size: json_l.grid_size,
            uid: json_l.uid,
        };

        layerdefs.insert(json_l.identifier.clone(), layerdef);
    }

    let resources = LdtkResources {
        levels: Vec::new(),
        tilesets: tilesets,
        layer_defs: layerdefs,
    };

    Ok(resources)
}
