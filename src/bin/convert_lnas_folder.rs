use clap::{App, Arg};
use std::collections::HashMap;
use std::path;
use std::path::PathBuf;
use stl2lnas::cfg;
use stl2lnas::cfg::Configs;
use stl2lnas::lagrangian;
use stl2lnas::stl;
use stl2lnas::stl::{surfaces::get_surfaces, triangle::TriangleSTL};

fn get_normalized_triangles(cfg: &Configs, triangles: &Vec<TriangleSTL>) -> Vec<TriangleSTL> {
    if cfg.normalization.is_some() {
        let triangles_norm = stl::triangle::normalize_triangles(
            &triangles,
            cfg.normalization.as_ref().unwrap().size as f32,
            &cfg.normalization.as_ref().unwrap().direction,
        );
        return triangles_norm;
    } else {
        return triangles.to_vec();
    }
}

fn generate_lnas(cfg: &Configs) {
    let (triangles, surfaces) = get_surfaces(&cfg.stl.files);
    let orig_triangles = get_normalized_triangles(&cfg, &triangles);

    let lagrangian_vertices = lagrangian::vertice::generate_lagrangian_vertices(&orig_triangles);
    let lagrangian_triangles =
        lagrangian::triangle::generate_lagrangian_triangles(&lagrangian_vertices, &orig_triangles);

    let (joined_vertices, joined_triangles) =
        lagrangian::join::join_information(&lagrangian_vertices, &lagrangian_triangles);
    let lnas_obj =
        lagrangian::format::get_lnas_obj_save(&cfg, &joined_vertices, &joined_triangles, &surfaces);

    let folder_path = path::Path::new(&cfg.output.folder);

    let lnas_filename = folder_path.join(format!("{}.lnas", cfg.name));
    lagrangian::save::save_lnas(lnas_filename.as_path(), &lnas_obj)
        .unwrap_or_else(|e| panic!("Saving lnas error. Error: {}", e));
}

fn get_configs(filename_cfg: &str) -> Configs {
    let mut cfg = cfg::Configs::new(filename_cfg).unwrap();
    // Update configs
    let path = PathBuf::from(filename_cfg);
    let dir = path.parent().unwrap();
    cfg.output.folder = dir.to_str().unwrap().to_string();
    let mut new_files: HashMap<String, String> = HashMap::new();
    for (surface_name, filename) in cfg.stl.files.into_iter() {
        let path_buff = PathBuf::from(filename);
        let filename_stl = path_buff.file_name().unwrap();
        let new_path_stl = dir.join(filename_stl);
        new_files.insert(surface_name, new_path_stl.to_str().unwrap().to_string());
    }
    cfg.stl.files = new_files;
    return cfg;
}

fn main() {
    let cli_app = App::new("stl2lnas")
        .author("Waine Oliveira Junior <waine@aerosim.io>")
        .about("Converts STL files to LNAS (Lagrangian Nassu format)")
        .arg(
            Arg::with_name("cfg")
                .short("c")
                .long("cfg")
                .value_name("YAML_FILE")
                .help("Configuration file for conversion")
                .required(true),
        );

    let matches = cli_app.get_matches();
    let filename_cfg = matches.value_of("cfg").unwrap();
    let cfg = get_configs(filename_cfg);

    generate_lnas(&cfg);
    println!("Generated!");
}
