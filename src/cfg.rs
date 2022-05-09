use crate::utils::create_folder_for_filename;
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
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ConfigsConversion {
    pub normalization_x: f32,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Configs {
    pub stl: ConfigsSTL,
    pub name: String,
    pub conversion: ConfigsConversion,
    pub output: ConfigsOutput,
}

impl Configs {
    pub fn new(filename: &str) -> Result<Configs, Box<dyn Error>> {
        let f: Configs = serde_yaml::from_reader(fs::File::open(filename)?)?;
        return Ok(f);
    }

    fn save_stl_to_output_folder(&self) -> Result<(), Box<dyn Error>> {
        let filename = path::Path::new(self.output.folder.as_str());
        fs::copy(
            &self.stl.filename,
            filename.join(format!("{}.stl", self.name)),
        )?;
        return Ok(());
    }

    pub fn save_to_output_folder(&self) -> Result<(), Box<dyn Error>> {
        let filename = path::Path::new(self.output.folder.as_str());
        let filename = filename.join("cfg.yaml");
        create_folder_for_filename(filename.as_path())?;
        let file = fs::File::create(filename.as_path())?;
        serde_yaml::to_writer(file, self)?;

        self.save_stl_to_output_folder()?;
        return Ok(());
    }
}
