use serde::{Deserialize, Serialize};
use serde_yaml;
use std::error::Error;
use std::{fs, path, string::String};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ConfigsSTL {
    pub filename: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ConfigsOutput {
    pub folder: String,
    pub save_csv: bool,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ConfigsLvl0 {
    pub x_size: f32,
    pub min_area: f32,
    pub max_area: f32,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ConfigsConversion {
    pub lvls_generate: Vec<u8>,
    pub lvl0: ConfigsLvl0,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Configs {
    pub stl: ConfigsSTL,
    pub conversion: ConfigsConversion,
    pub output: ConfigsOutput,
}

impl Configs {
    pub fn new(filename: &str) -> Result<Configs, Box<dyn Error>> {
        let f: Configs = serde_yaml::from_reader(fs::File::open(filename)?)?;
        return Ok(f);
    }

    pub fn save_to_output_folder(&self) -> Result<(), Box<dyn Error>> {
        let filename = path::Path::new(self.output.folder.as_str());
        let filename = filename.join("cfg.yaml");
        let file = fs::File::create(filename.as_path())?;
        serde_yaml::to_writer(file, self)?;
        return Ok(());
    }
}
