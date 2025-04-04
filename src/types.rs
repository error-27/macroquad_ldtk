use std::collections::HashMap;

use macroquad::color::Color;

pub struct LdtkResources {
    pub levels: Vec<LdtkLevel>,

    /// Map of all tilesets
    pub tilesets: HashMap<String, LdtkTileset>,

    pub layer_defs: HashMap<String, LdtkLayerDef>,
}

pub struct LdtkLevel {
    pub layers: Vec<LdtkLayerInstance>,
}

pub struct LdtkLayerDef {
    pub layer_type: LdtkLayerType,
    pub identifier: String,
    pub opacity: f64,
    pub grid_size: i64,

    pub uid: i64,
}

pub struct LdtkLayerInstance {
    pub grid_height: i64,
    pub grid_width: i64,
    pub grid_size: i64,

    /// Identifier used to index through the hashmap of layer definitions
    pub layerdef_id: String,

    /// Path of tileset, used to index into a hashmap
    pub tileset_id: String,

    pub tiles: Vec<LdtkTileInstance>,
}

pub struct LdtkEntityDef {
    pub allow_out_of_bounds: bool,
    pub color: Color,
    pub height: i64,
    pub width: i64,

    /// Unique identifier
    pub uid: i64,

    /// User-defined identifier
    pub identifier: String,
}

pub struct LdtkTileset {
    /// Index of the texture in the passed-in array
    pub texture_index: u32,

    pub grid_height: i64,
    pub grid_width: i64,

    pub padding: i64,
    pub spacing: i64,
    pub tile_grid_size: i64,

    pub identifier: String,
    pub uid: i64,
}

pub struct LdtkTileInstance {
    pub alpha: f64,

    pub px_coords: [i64; 2],

    pub src_coords: [i64; 2],

    pub tile_id: i64,
}

pub struct LdtkTileRect {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,

    pub tileset_uid: i64,
}

pub enum LdtkLayerType {
    IntGrid,
    Entities,
    Tiles,
    AutoLayer,
}
