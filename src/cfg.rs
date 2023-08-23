use crate::utils::create_folder_for_filename;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::error::Error;
use std::{collections::HashMap, fs, path, string::String};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ConfigsSTL {
    pub files: HashMap<String, String>,
    pub folders: Vec<String>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ConfigsOutput {
    pub folder: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ConfigsNormalization {
    pub size: f32,
    pub direction: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Configs {
    pub stl: ConfigsSTL,
    pub name: String,
    pub normalization: Option<ConfigsNormalization>,
    pub output: ConfigsOutput,
}

impl Configs {
    pub fn new(filename: &str) -> Result<Configs, Box<dyn Error>> {
        let f: Configs = serde_yaml::from_reader(fs::File::open(filename)?)?;

        if f.normalization.is_some() {
            if !["x", "y", "z"].contains(&f.normalization.as_ref().unwrap().direction.as_str()) {
                panic!("normalization direction must be 'x', 'y' or 'z'");
            }
        }
        for (_name, stl_filename) in f.all_stls().iter() {
            if (!stl_filename.exists() || !stl_filename.is_file()) {
                panic!(
                    "STL path {:?} doesn't exists or is not a file",
                    stl_filename
                );
            }
            if (!stl_filename.to_str().unwrap().ends_with(".stl")) {
                panic!("STL path {:?} doesn't end with .stl", stl_filename);
            }
        }
        return Ok(f);
    }

    pub fn folder_stls(&self, foldername: &str) -> Vec<path::PathBuf> {
        let paths = fs::read_dir(foldername).unwrap();
        let mut all_paths: Vec<path::PathBuf> = Vec::new();
        for path in paths {
            let full_path = path.unwrap().path();
            if full_path.to_str().unwrap().ends_with(".stl") && full_path.is_file() {
                all_paths.push(full_path.to_path_buf());
            }
        }
        return all_paths;
    }

    pub fn all_stls(&self) -> HashMap<String, path::PathBuf> {
        let mut stls: HashMap<String, path::PathBuf> = HashMap::new();
        for foldername in self.stl.folders.iter() {
            let paths = self.folder_stls(foldername);
            for p in paths.iter() {
                let name = p.file_name().unwrap().to_str().unwrap();
                if (stls.contains_key(name)) {
                    panic!("Repeated name {} in folder STLs", name);
                }
                stls.insert(name.to_string(), p.to_owned());
            }
        }
        for (name, stl_filename) in self.stl.files.iter() {
            if (stls.contains_key(name)) {
                panic!("Repeated name {} in file STLs", name);
            }
            let path = path::Path::new(stl_filename);
            stls.insert(name.to_owned(), path.to_owned());
        }
        return stls;
    }

    fn save_stl_to_output_folder(&self) -> Result<(), Box<dyn Error>> {
        let foldername = path::Path::new(self.output.folder.as_str());
        for (stl_name, stl_filename) in self.all_stls().iter() {
            fs::copy(stl_filename, foldername.join(format!("{}.stl", stl_name)))?;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn read_filename(filename: &str) -> Configs {
        let filename_cfg = filename.to_string();
        let cfg = Configs::new(&filename_cfg).unwrap();
        return cfg;
    }

    #[test]
    fn check_examples_files() {
        read_filename("examples/convert_folder.yaml");
        read_filename("examples/convert_cube.yaml");
        read_filename("examples/convert_cylinder.yaml");
        read_filename("examples/convert_plane.yaml");
        read_filename("examples/convert_sphere.yaml");
        read_filename("examples/convert_terrain.yaml");
        read_filename("examples/convert_cube_plane.yaml");
        read_filename("examples/convert_cube_no_norm.yaml");
    }

    #[test]
    fn check_convert_folder() {
        let cfg = read_filename("examples/convert_folder.yaml");
        let stls = cfg.all_stls();
        println!("a {:?}", stls);
    }
}
