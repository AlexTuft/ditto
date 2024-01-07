mod common;
mod data_utils;
mod gen1;
pub mod gen2;
mod gen3;

use common::Save;
use std::io;

pub fn read_save(path: &str) -> io::Result<Save> {
    let data = std::fs::read(path)?;
    Ok(Save { data })
}
