use macroquad::{color::Color, texture::Texture2D};

pub struct LdtkResources {
    pub levels: Vec<LdtkLevel>,

    /// Vec of all tilesets
    pub tilesets: Vec<LdtkTileset>,
}

pub struct LdtkLevel {}

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
    pub texture: Texture2D,

    pub grid_height: i64,
    pub grid_width: i64,

    pub padding: i64,
    pub spacing: i64,
    pub tile_grid_size: i64,

    pub identifier: String,
    pub uid: i64,
}

pub struct LdtkTileRect {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,

    pub tileset_uid: i64,
}
