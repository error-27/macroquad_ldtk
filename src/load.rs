use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use macroquad::texture::Texture2D;

use crate::{
    parser::*,
    types::{LdtkLayerDef, LdtkLayerInstance, LdtkLayerType, LdtkLevel, LdtkTileInstance},
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

    let mut levels: Vec<LdtkLevel> = Vec::new();
    for level in &json.levels {
        let mut layer_insts: Vec<LdtkLayerInstance> = Vec::new();

        for l in level.layer_instances.as_ref().unwrap() {
            let tiles: Vec<LdtkTileInstance> = l
                .grid_tiles
                .iter()
                .map(|me| convert::convert_tile_instance(me))
                .collect();
            let l_converted = LdtkLayerInstance {
                grid_height: l.c_hei,
                grid_width: l.c_wid,
                grid_size: l.grid_size,
                layerdef_id: l.identifier.clone(),
                tileset_id: l.tileset_rel_path.clone().unwrap(),
                grid_tiles: tiles,
            };
            layer_insts.push(l_converted);
        }

        levels.push(LdtkLevel {
            layers: layer_insts,
        });
    }

    let resources = LdtkResources {
        levels: levels,
        tilesets: tilesets,
        layer_defs: layerdefs,
    };

    Ok(resources)
}

mod convert {
    use crate::parser::TileInstance;
    use crate::types::LdtkTileInstance;
    pub fn convert_tile_instance(source: &TileInstance) -> LdtkTileInstance {
        LdtkTileInstance {
            alpha: source.a,
            px_coords: [source.px[0], source.px[1]],
            src_coords: [source.src[0], source.src[1]],
            tile_id: source.t,
        }
    }
}
