use crate::stl_indexed;

pub fn divide_stl_by_area(max_area: f64, mut indexed_stl: &stl_indexed::IndexedSTL) {}

pub fn divide_all_triangles(indexed_stl: &mut stl_indexed::IndexedSTL) {
    let orig_triangles = indexed_stl.triangles.clone();
    for t in orig_triangles {
        indexed_stl.divide_triangle(t);
    }
}
