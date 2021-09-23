use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fs, string::String, path};
use std::error::Error;

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ConversionConfigs {
    pub path_stl: String,
    pub folder_output: String,
    pub size_x_lvl0: f32,
    pub lvls_generate: Vec<u8>,
    pub save_csv: bool,
}

impl ConversionConfigs{
    pub fn new(filename: &str) -> Result<ConversionConfigs, Box<dyn Error>> {
        let f: ConversionConfigs = serde_yaml::from_reader(fs::File::open(filename)?)?;
        return Ok(f);
    }
    
    pub fn save_to_output_folder(&self) -> Result<(), Box<dyn Error>>{
        let filename = path::Path::new(self.folder_output.as_str());
        let filename = filename.join("cfg.yaml");
        let file = fs::File::create(filename.as_path())?;
        serde_yaml::to_writer(file, self)?;
        return Ok(());
    }
}

