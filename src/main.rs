pub mod cfg;
pub mod lagrangian_node;
pub mod lagrangian_save;
pub mod stl_divider;
pub mod stl_reader;
pub mod stl_triangle;
pub mod utils;

use cfg::Configs;
use clap::{App, Arg};
use lagrangian_node::LagrangianNode;
use std::path;
use stl_triangle::TriangleSTL;

fn get_min_max_area_for_lvl(cfg: &Configs, lvl: u8) -> (f64, f64) {
    // Factor to wich divide lvl 0 area.
    // At each level, dist is divided by 2, so area is divided by 4 (2^2)
    let area_factor = 4u32.pow(lvl as u32) as f32;
    let (min_area, max_area) = (
        cfg.conversion.lvl0.min_area / area_factor,
        cfg.conversion.lvl0.max_area / area_factor,
    );
    return (min_area as f64, max_area as f64);
}

fn get_normalized_triangles(cfg: &Configs) -> Vec<TriangleSTL> {
    let triangles = stl_reader::read_stl(cfg.stl.filename.as_str());
    let triangles =
        stl_triangle::normalize_triangles(&triangles, cfg.conversion.lvl0.x_size as f64);
    return triangles;
}

fn generate_lagrangian_nodes_for_lvl(
    cfg: &Configs,
    lvl: u8,
    orig_triangles: &Vec<TriangleSTL>,
) -> Vec<LagrangianNode> {
    let (min_area, max_area) = get_min_max_area_for_lvl(cfg, lvl);
    let mut div_stl = stl_divider::DividerSTL::new(orig_triangles.clone());
    div_stl.divide_stl_by_area(max_area, min_area);

    let lagrangian_nodes = lagrangian_node::stl2lagrangian(div_stl.triangles);
    return lagrangian_nodes;
}

fn save_nodes_for_lvl(cfg: &Configs, lvl: u8, lagrangian_nodes: &Vec<LagrangianNode>) {
    let folder_path = path::Path::new(&cfg.output.folder);
    let (min_area, max_area) = get_min_max_area_for_lvl(cfg, lvl);

    let lnas_filename = folder_path.join(format!("lvl{:02}.lnas", lvl));
    lagrangian_save::save_lagrangian_nodes_lnas(
        lnas_filename.as_path(),
        lagrangian_nodes,
        min_area as f32,
        max_area as f32,
    )
    .unwrap_or_else(|e| println!("Saving lnas error for lvl {}. Error: {}", lvl, e));

    if cfg.output.save_csv {
        let csv_filename = folder_path.join(format!("lvl{:02}.csv", lvl));
        lagrangian_save::save_lagrangian_nodes_csv(csv_filename.as_path(), lagrangian_nodes)
            .unwrap_or_else(|e| println!("Saving csv error for lvl {}. Error: {}", lvl, e));
    }
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
    for lvl in cfg.conversion.lvls_generate.iter() {
        let lagrangian_nodes = generate_lagrangian_nodes_for_lvl(&cfg, *lvl, &orig_triangles);
        save_nodes_for_lvl(&cfg, *lvl, &lagrangian_nodes);
        println!("Generated level {}!", lvl);
    }
}
