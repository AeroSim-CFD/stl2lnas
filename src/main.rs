mod lagrangian {
    pub mod format;
    pub mod save;
    pub mod triangle;
    pub mod vertice;
}

pub mod cfg;
pub mod stl_reader;
pub mod stl_triangle;
pub mod utils;

use cfg::Configs;
use clap::{App, Arg};
use lagrangian::triangle::LagrangianTriangle;
use lagrangian::vertice::LagrangianVertice;
use std::path;
use stl_triangle::TriangleSTL;

fn get_normalized_triangles(cfg: &Configs) -> Vec<TriangleSTL> {
    let triangles = stl_reader::read_stl(cfg.stl.filename.as_str());
    let triangles =
        stl_triangle::normalize_triangles(&triangles, cfg.conversion.normalization_x as f32);
    return triangles;
}

fn save_lnas(
    cfg: &Configs,
    lagrangian_vertices: &Vec<LagrangianVertice>,
    lagrangian_triangles: &Vec<LagrangianTriangle>,
) {
    let folder_path = path::Path::new(&cfg.output.folder);

    let lnas_filename = folder_path.join(format!("{}.lnas", cfg.name));
    lagrangian::save::save_lagrangian_nodes_lnas(
        lnas_filename.as_path(),
        cfg,
        lagrangian_vertices,
        lagrangian_triangles,
    )
    .unwrap_or_else(|e| println!("Saving lnas error. Error: {}", e));
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

    cfg.save_to_output_folder()
        .unwrap_or_else(|e| println!("Unable to save configs in its output folder. Error: {}", e));

    let orig_triangles = get_normalized_triangles(&cfg);
    let lagrangian_vertices = lagrangian::vertice::generate_lagrangian_vertices(&orig_triangles);
    let lagrangian_triangles =
        lagrangian::triangle::generate_lagrangian_triangles(&lagrangian_vertices, &orig_triangles);
    save_lnas(&cfg, &lagrangian_vertices, &lagrangian_triangles);
    println!("Generated!");
}
