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
    let mut idx_stl = stl_indexed::IndexedSTL::new(normalized_triangles);
    for point in idx_stl.points.iter() {
        println!("points {}", point);
    }
    println!("bef {} {}", idx_stl.points.len(), idx_stl.triangles.len());
    for t in idx_stl.triangles.iter() {
        println!("area b {} {}", t.area(), t);
    }
    // stl_divider::divide_all_triangles(&mut idx_stl);
    println!("after {} {}", idx_stl.points.len(), idx_stl.triangles.len());
    for t in idx_stl.triangles.iter() {
        println!("area a {} {}", t.area(), t);
    }
    for p in idx_stl.points.iter() {
        println!("p {} ", p);
    }
    let max_area = 1000f64;
    let min_area = 100f64;
    stl_divider::divide_stl_by_area(max_area, min_area, &mut idx_stl);
    println!("after after {} {}", idx_stl.points.len(), idx_stl.triangles.len());
    for t in idx_stl.triangles.iter() {
        println!("area aa {} {}", t.area(), t);
    }
    for p in idx_stl.points.iter() {
        println!("pp {} ", p);
    }
}
