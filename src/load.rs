use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use convert::{convert_layer_def, convert_level};
use macroquad::texture::Texture2D;

use crate::{
    parser::*,
    types::{LdtkLayerDef, LdtkLayerInstance, LdtkLevel, LdtkTileInstance},
};

use super::types::{LdtkResources, LdtkTileset};

/// Loads an LDtk project from a JSON file.
/// Returns a struct containing the LDtk project resources.
pub async fn load_project(path: &str, textures: &[(Texture2D, &str)]) -> io::Result<LdtkResources> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json: LdtkJson = serde_json::from_reader(reader)?;

    let mut path_base = PathBuf::from(path);
    path_base.pop(); // Remove the filename so just the folder containing the project remains

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
    let mut layer_defs: HashMap<String, LdtkLayerDef> = HashMap::new();
    for json_l in &json.defs.layers {
        let layerdef = convert_layer_def(json_l).expect("failed to convert layerdef"); // TODO: pass error up the stack

        layer_defs.insert(json_l.identifier.clone(), layerdef);
    }

    // Load levels
    let mut levels: Vec<LdtkLevel> = Vec::new();
    for level in &json.levels {
        levels.push(convert_level(level));
    }

    // Compile loaded assets into the final structure
    let resources = LdtkResources {
        levels,
        tilesets,
        layer_defs,
    };

    Ok(resources)
}

mod convert {
    use crate::parser::{LayerDefinition, Level, TileInstance};
    use crate::types::{
        LdtkLayerDef, LdtkLayerInstance, LdtkLayerType, LdtkLevel, LdtkTileInstance,
    };

    /// Converts a TileInstance into an LdtkTileInstance.
    pub fn convert_tile_instance(input: &TileInstance) -> LdtkTileInstance {
        LdtkTileInstance {
            alpha: input.a,
            px_coords: [input.px[0], input.px[1]],
            src_coords: [input.src[0], input.src[1]],
            tile_id: input.t,
        }
    }

    /// Converts a String into the correct LdtkLayerType.
    pub fn convert_layer_type(input: &String) -> Result<LdtkLayerType, String> {
        match input.as_str() {
            "IntGrid" => Ok(LdtkLayerType::IntGrid),
            "Tiles" => Ok(LdtkLayerType::Tiles),
            "AutoLayer" => Ok(LdtkLayerType::AutoLayer),
            "Entities" => Ok(LdtkLayerType::Entities),
            _ => Err("invalid layer type".to_owned()),
        }
    }

    /// Converts LayerDefinition to an LdtkLayerDef.
    pub fn convert_layer_def(input: &LayerDefinition) -> Result<LdtkLayerDef, String> {
        let layer_type = convert_layer_type(&input.layer_definition_type)?;

        let layerdef = LdtkLayerDef {
            layer_type,
            identifier: input.identifier.clone(),
            opacity: input.display_opacity,
            grid_size: input.grid_size,
            uid: input.uid,
        };

        Ok(layerdef)
    }

    /// Converts a Level into an LdtkLevel.
    pub fn convert_level(input: &Level) -> LdtkLevel {
        let mut layer_insts: Vec<LdtkLayerInstance> = Vec::new();

        for l in input.layer_instances.as_ref().unwrap() {
            let tiles: Vec<LdtkTileInstance> = l
                .grid_tiles
                .iter()
                .map(|me| convert_tile_instance(me))
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

        LdtkLevel {
            layers: layer_insts,
        }
    }
}
