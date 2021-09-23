use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{error::Error, fs, string::String};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ConversionConfigs {
    pub path_stl: String,
    pub folder_output: String,
    pub size_x_lvl0: f32,
    pub lvls_generate: Vec<u8>,
}

pub fn new_from_file(filename: &str) -> Result<ConversionConfigs, Box<dyn Error>> {
    let f: ConversionConfigs = serde_yaml::from_reader(fs::File::open(filename)?)?;
    return Ok(f);
}
