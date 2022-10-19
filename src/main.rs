pub mod lagrangian {
    pub mod format;
    pub mod join;
    pub mod save;
    pub mod triangle;
    pub mod vertice;
}

pub mod stl {
    pub mod reader;
    pub mod triangle;
}
pub mod cfg;
pub mod utils;

use cfg::Configs;
use clap::{App, Arg};
use std::path;
use stl::triangle::TriangleSTL;

fn get_normalized_triangles(cfg: &Configs) -> Vec<TriangleSTL> {
    let triangles = stl::reader::read_stl(cfg.stl.filename.as_str());
    let triangles = stl::triangle::normalize_triangles(
        &triangles,
        cfg.normalization.size as f32,
        &cfg.normalization.direction,
    );
    return triangles;
}

fn generate_lnas(cfg: &Configs) {
    cfg.save_to_output_folder()
        .unwrap_or_else(|e| println!("Unable to save configs in its output folder. Error: {}", e));

    let orig_triangles = get_normalized_triangles(&cfg);

    let lagrangian_vertices = lagrangian::vertice::generate_lagrangian_vertices(&orig_triangles);
    let lagrangian_triangles =
        lagrangian::triangle::generate_lagrangian_triangles(&lagrangian_vertices, &orig_triangles);

    let (joined_vertices, joined_triangles) =
        lagrangian::join::join_information(&lagrangian_vertices, &lagrangian_triangles);
    let lnas_obj = lagrangian::format::get_lnas_obj_save(&cfg, &joined_vertices, &joined_triangles);

    let folder_path = path::Path::new(&cfg.output.folder);

    let lnas_filename = folder_path.join(format!("{}.lnas", cfg.name));
    lagrangian::save::save_lnas(lnas_filename.as_path(), &lnas_obj)
        .unwrap_or_else(|e| panic!("Saving lnas error. Error: {}", e));
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

    let cfg = cfg::Configs::new(filename_cfg).unwrap();
    cfg.save_to_output_folder().unwrap();

    generate_lnas(&cfg);
    println!("Generated!");
}
