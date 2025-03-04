use std::{
    fs::File,
    io::{self, BufReader},
};

use crate::ldtk::parser::*;

/// Loads an LDtk project from a JSON file.
pub fn load_json(path: &str) -> io::Result<LdtkJson> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let parsed: LdtkJson = serde_json::from_reader(reader)?;

    Ok(parsed)
}
