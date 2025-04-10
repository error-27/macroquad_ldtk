//! `macroquad_ldtk` is a simple library to load LDtk projects for your `macroquad` projects.
//! `macroquad_ldtk` handles loading the project, trimming editor data, and rendering levels.
//! Other tasks like collision and entity spawning are left up to the user.

pub mod error;
pub mod load;
pub mod types;

mod levels;
mod parser;

pub mod prelude {
    pub use crate::error::Error;
    pub use crate::load::*;
    pub use crate::types::LdtkResources;
}
