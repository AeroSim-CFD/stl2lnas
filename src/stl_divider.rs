use crate::{utils, stl_triangle::TriangleSTL};
use core::panic;
use std::collections::HashSet;

pub struct DividerSTL {
    pub triangles: HashSet<TriangleSTL>,
}

fn get_points_triangle_division(
    triangle: TriangleSTL,
) -> [(utils::Vec3f, utils::Vec3f, utils::Vec3f); 4] {
    let (point0, point1, point2) = (triangle.point0, triangle.point1, triangle.point2);

    // Points between triangle vertices
    let mut point01 = point0 + point1;
    let mut point02 = point0 + point2;
    let mut point12 = point1 + point2;
    point01.divide(2f64);
    point02.divide(2f64);
    point12.divide(2f64);

    return [
        (point01, point02, point12), // middle triangle
        (point0, point01, point02),  // triangle with point 0
        (point1, point01, point12),  // triangle with point 1
        (point2, point02, point12),  // triangle with point 2
    ];
}

impl DividerSTL {
    pub fn new(triangles: Vec<TriangleSTL>) -> DividerSTL {
        let mut div_stl = DividerSTL {
            triangles: HashSet::new(),
        };
        for t in triangles {
            div_stl.add_triangle(t);
        }
        return div_stl;
    }

    fn add_triangle(&mut self, triangle: TriangleSTL) {
        self.triangles.insert(triangle);
    }

    fn remove_triangle(&mut self, triangle: TriangleSTL) {
        self.triangles.remove(&triangle);
    }

    pub fn divide_triangle(&mut self, triangle: TriangleSTL) -> [TriangleSTL; 4] {
        self.remove_triangle(triangle);

        let new_triangles_points = get_points_triangle_division(triangle);
        let mut vec_new_triangle: Vec<TriangleSTL> = Vec::new();
        for t_p in new_triangles_points {
            let new_triangle: TriangleSTL = TriangleSTL::new(t_p.0, t_p.1, t_p.2, triangle.normal);
            self.add_triangle(new_triangle);
            vec_new_triangle.push(new_triangle);
        }
        return [
            vec_new_triangle[0],
            vec_new_triangle[1],
            vec_new_triangle[2],
            vec_new_triangle[3],
        ];
    }
}

fn get_triangles_by_min_area(min_area: f64, div_stl: &mut DividerSTL) -> Vec<TriangleSTL> {
    let triangles_lower_area: Vec<TriangleSTL> = div_stl
        .triangles
        .iter()
        .filter(|t| t.area() < min_area)
        .cloned()
        .collect();
    return triangles_lower_area;
}

fn divide_stl_by_min_area(min_area: f64, div_stl: &mut DividerSTL) {
    let triangles_lower_area = get_triangles_by_min_area(min_area, div_stl);
    while triangles_lower_area.len() > 0 {
        panic!("I don't know how to increase area of triangles yet :(");
    }
}

fn get_triangles_by_max_area(max_area: f64, div_stl: &mut DividerSTL) -> Vec<TriangleSTL> {
    let triangles_greater_area: Vec<TriangleSTL> = div_stl
        .triangles
        .iter()
        .filter(|t| t.area() > max_area)
        .cloned()
        .collect();
    return triangles_greater_area;
}

fn divide_stl_by_max_area(max_area: f64, div_stl: &mut DividerSTL) {
    let mut triagles_2_divide = get_triangles_by_max_area(max_area, div_stl);

    // While there are still triangles to refine
    while triagles_2_divide.len() > 0 {
        let mut new_triangles_2_divide: Vec<TriangleSTL> = Vec::new();
        for t in triagles_2_divide.iter() {
            let created_triangles = div_stl.divide_triangle(*t);
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

pub fn divide_stl_by_area(max_area: f64, min_area: f64, div_stl: &mut DividerSTL) {
    if max_area < min_area * 4f64 {
        panic!(
            "Max area should be at least 4 times greater than min area. max:{:.4} min:{:.4}",
            max_area, min_area
        )
    }

    divide_stl_by_max_area(max_area, div_stl);
    divide_stl_by_min_area(min_area, div_stl);
}

pub fn divide_all_triangles(div_stl: &mut DividerSTL) {
    let orig_triangles = div_stl.triangles.clone();
    for t in orig_triangles {
        div_stl.divide_triangle(t);
    }
}
