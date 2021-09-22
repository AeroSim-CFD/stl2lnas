pub mod common;
pub mod stl_indexed;
pub mod stl_reader;
pub mod stl_triangle;

fn main() {
    let filename = "fixture/cube.stl";
    let triangles = stl_reader::read_stl(filename);
    for triangle in triangles.iter() {
        println!("1 {}", triangle);
    }
    let normalized_triangles = stl_triangle::normalize_triangles(&triangles);
    for triangle in normalized_triangles.iter() {
        println!("2 {}", triangle);
    }
    let indexed_triangles = stl_indexed::IndexedSTL::new(normalized_triangles);
    for point in indexed_triangles.points.iter() {
        println!("points {}", point);
    }
}
