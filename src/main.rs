pub mod common;
pub mod stl_divider;
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
    let mut indexed_stl = stl_indexed::IndexedSTL::new(normalized_triangles);
    for point in indexed_stl.points.iter() {
        println!("points {}", point);
    }
    println!("bef {} {}", indexed_stl.points.len(), indexed_stl.triangles.len());
    for t in indexed_stl.triangles.iter() {
        println!("area b {} {}", t.area(), t);
    }
    stl_divider::divide_all_triangles(&mut indexed_stl);
    println!("after {} {}", indexed_stl.points.len(), indexed_stl.triangles.len());
    for t in indexed_stl.triangles.iter() {
        println!("area a {} {}", t.area(), t);
    }
}
