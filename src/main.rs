pub mod common;
pub mod lagrangian_node;
pub mod lagrangian_save;
pub mod stl2lagrangian;
pub mod stl_divider;
pub mod stl_indexed;
pub mod stl_reader;
pub mod stl_triangle;

fn main() {
    let filename = "fixture/cube.stl";
    let total_dist_x = 2.5f64;
    let triangles = stl_reader::read_stl(filename);
    // for triangle in triangles.iter() {
    //     println!("1 {}", triangle);
    // }
    let normalized_triangles = stl_triangle::normalize_triangles(&triangles, total_dist_x);
    // for triangle in normalized_triangles.iter() {
    //     println!("2 {}", triangle);
    // }
    let mut idx_stl = stl_indexed::IndexedSTL::new(normalized_triangles);

    // for t in idx_stl.triangles.iter() {
    //     println!("area b {} {}", t.area(), t);
    // }
    // // stl_divider::divide_all_triangles(&mut idx_stl);
    // for t in idx_stl.triangles.iter() {
    //     println!("area a {} {}", t.area(), t);
    // }
    let lvl: u32 = 6;
    let area_factor = 4u32.pow(lvl) as f64;
    let max_area = 2f64 / area_factor;
    let min_area = 0.2f64 / area_factor;
    stl_divider::divide_stl_by_area(max_area, min_area, &mut idx_stl);
    // for t in idx_stl.triangles.iter() {
    //     println!("area aa {} {}", t.area(), t);
    // }

    let lagrangian_nodes = stl2lagrangian::stl2lagrangian(&idx_stl);
    // for l in lagrangian_nodes.iter() {
    //     println!("l {}", l)
    // }
    println!("len l {}", lagrangian_nodes.len());
}
