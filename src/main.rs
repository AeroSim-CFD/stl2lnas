pub mod cfg;
pub mod lagrangian_node;
pub mod lagrangian_save;
pub mod stl2lagrangian;
pub mod stl_divider;
pub mod stl_reader;
pub mod stl_triangle;
pub mod utils;
use std::path;

use cfg::ConversionConfigs;
use lagrangian_node::LagrangianNode;
use stl_triangle::TriangleSTL;

const MAX_AREA_LVL0: f64 = 1f64;
const MIN_AREA_LVL0: f64 = 0.2f64;

fn get_min_max_area_for_lvl(lvl: u8) -> (f64, f64) {
    // Factor to wich divide lvl 0 area.
    // At each level, dist is divided by 2, so area is divided by 4 (2^2)
    let area_factor = 4u32.pow(lvl as u32) as f64;
    let (min_area, max_area) = (MIN_AREA_LVL0 / area_factor, MAX_AREA_LVL0 / area_factor);
    return (min_area, max_area);
}

fn get_normalized_triangles(conv_cfg: &ConversionConfigs) -> Vec<TriangleSTL> {
    let triangles = stl_reader::read_stl(conv_cfg.path_stl.as_str());
    let triangles = stl_triangle::normalize_triangles(&triangles, conv_cfg.size_x_lvl0 as f64);
    return triangles;
}

fn generate_lagrangian_nodes_for_lvl(
    conv_cfg: &ConversionConfigs,
    lvl: u8,
    orig_triangles: &Vec<TriangleSTL>,
) -> Vec<LagrangianNode> {
    let (min_area, max_area) = get_min_max_area_for_lvl(lvl);
    let mut div_stl = stl_divider::DividerSTL::new(orig_triangles.clone());
    div_stl.divide_stl_by_area(max_area, min_area);

    let lagrangian_nodes = stl2lagrangian::stl2lagrangian(div_stl.triangles);
    return lagrangian_nodes;
}

fn save_nodes_for_lvl(
    conv_cfg: &ConversionConfigs,
    lvl: u8,
    lagrangian_nodes: &Vec<LagrangianNode>,
) {
    let folder_path = path::Path::new(&conv_cfg.folder_output);
    let (min_area, max_area) = get_min_max_area_for_lvl(lvl);

    let lnas_filename = folder_path.join(format!("lvl{:02}.lnas", lvl));
    lagrangian_save::save_lagrangian_nodes_lnas(
        lnas_filename.as_path(),
        lagrangian_nodes,
        min_area as f32,
        max_area as f32,
    )
    .unwrap_or_else(|e| println!("Saving lnas error for lvl {}. Error: {}", lvl, e));

    if conv_cfg.save_csv {
        let csv_filename = folder_path.join(format!("lvl{:02}.csv", lvl));
        lagrangian_save::save_lagrangian_nodes_csv(csv_filename.as_path(), lagrangian_nodes)
            .unwrap_or_else(|e| println!("Saving csv error for lvl {}. Error: {}", lvl, e));
    }
}


fn main() {
    let filename_cfg = "fixture/convert_cube.yaml";
    let conv_cfg = cfg::ConversionConfigs::new(filename_cfg).unwrap();

    conv_cfg.save_to_output_folder().unwrap_or_else(
        |e| println!("Unable to save configs in its output folder. Error: {}", e));

    let orig_triangles = get_normalized_triangles(&conv_cfg);
    for lvl in conv_cfg.lvls_generate.iter() {
        let lagrangian_nodes = generate_lagrangian_nodes_for_lvl(
            &conv_cfg, *lvl, &orig_triangles);
        save_nodes_for_lvl(&conv_cfg, *lvl, &lagrangian_nodes);
        println!("Generated level {}!", lvl);
    }
}
