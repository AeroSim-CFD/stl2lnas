use core::panic;

use crate::{stl_indexed, stl_triangle};

fn get_triangles_by_min_area(
    min_area: f64,
    idx_stl: &mut stl_indexed::IndexedSTL,
) -> Vec<stl_triangle::TriangleSTL> {
    let triangles_lower_area: Vec<stl_triangle::TriangleSTL> = idx_stl
        .triangles
        .iter()
        .filter(|t| t.area() < min_area)
        .cloned()
        .collect();
    return triangles_lower_area;
}

fn divide_stl_by_min_area(min_area: f64, idx_stl: &mut stl_indexed::IndexedSTL) {
    let triangles_lower_area = get_triangles_by_min_area(min_area, idx_stl);
    while triangles_lower_area.len() > 0 {
        panic!("I don't know how to increase area of triangles yet :(");
    }
}

fn get_triangles_by_max_area(
    max_area: f64,
    idx_stl: &mut stl_indexed::IndexedSTL,
) -> Vec<stl_triangle::TriangleSTL> {
    let triangles_greater_area: Vec<stl_triangle::TriangleSTL> = idx_stl
        .triangles
        .iter()
        .filter(|t| t.area() > max_area)
        .cloned()
        .collect();
    return triangles_greater_area;
}

fn divide_stl_by_max_area(max_area: f64, idx_stl: &mut stl_indexed::IndexedSTL) {
    let mut triagles_2_divide = get_triangles_by_max_area(max_area, idx_stl);

    // While there are still triangles to refine
    while triagles_2_divide.len() > 0 {
        let mut new_triangles_2_divide: Vec<stl_triangle::TriangleSTL> = Vec::new();
        for t in triagles_2_divide.iter() {
            let created_triangles = idx_stl.divide_triangle(*t);
            // Check if some created triangles still need refinement
            for ct in created_triangles {
                if ct.area() > max_area {
                    new_triangles_2_divide.push(ct);
                }
            }
        }
        // Add new triangles to divide
        triagles_2_divide = new_triangles_2_divide.clone();
    }
}

pub fn divide_stl_by_area(max_area: f64, min_area: f64, idx_stl: &mut stl_indexed::IndexedSTL) {
    if max_area < min_area * 4f64 {
        panic!(
            "Max area should be at least 4 times greater than min area. max:{:.4} min:{:.4}",
            max_area, min_area
        )
    }

    divide_stl_by_max_area(max_area, idx_stl);
    divide_stl_by_min_area(min_area, idx_stl);
}

pub fn divide_all_triangles(idx_stl: &mut stl_indexed::IndexedSTL) {
    let orig_triangles = idx_stl.triangles.clone();
    for t in orig_triangles {
        idx_stl.divide_triangle(t);
    }
}
