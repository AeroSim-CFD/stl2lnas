use crate::common;
use crate::stl_triangle::{self, TriangleSTL};
use std::collections::HashSet;

pub struct IndexedSTL {
    pub points: HashSet<common::Vec3f>,
    pub normals: HashSet<common::Vec3f>,
    pub triangles: Vec<stl_triangle::TriangleSTL>,
}

fn get_points_triangle_division(
    triangle: stl_triangle::TriangleSTL,
) -> [(common::Vec3f, common::Vec3f, common::Vec3f); 4] {
    let (point0, point1, point2) = (triangle.point0, triangle.point1, triangle.point2);

    // Points between triangle vertices
    let mut point01 = point0 + point1;
    let mut point02 = point0 + point2;
    let mut point12 = point1 + point2;
    point01.divide(2f32);
    point02.divide(2f32);
    point12.divide(2f32);

    return [
        (point01, point02, point12), // middle triangle
        (point0, point01, point02),  // triangle with point 0
        (point1, point01, point12),  // triangle with point 1
        (point2, point02, point12),  // triangle with point 2
    ];
}

impl IndexedSTL {
    pub fn new(triangles: Vec<stl_triangle::TriangleSTL>) -> IndexedSTL {
        let mut idx_stl =
            IndexedSTL { points: HashSet::new(), normals: HashSet::new(), triangles: Vec::new() };
        for t in triangles {
            idx_stl.add_triangle(t);
        }
        return idx_stl;
    }

    fn add_triangle(&mut self, triangle: stl_triangle::TriangleSTL) {
        self.add_point(triangle.point0);
        self.add_point(triangle.point1);
        self.add_point(triangle.point2);
        self.add_normal(triangle.normal);

        self.triangles.push(triangle);
    }

    fn remove_triangle(&mut self, triangle: stl_triangle::TriangleSTL) {
        self.triangles.swap_remove(
            self.triangles.iter().position(|x| *x == triangle).expect("Triangle not found"),
        );
    }

    fn add_point(&mut self, point: common::Vec3f) {
        self.points.insert(point);
    }

    fn add_normal(&mut self, normal: common::Vec3f) {
        self.normals.insert(normal);
    }

    pub fn divide_triangle(
        &mut self,
        triangle: stl_triangle::TriangleSTL,
    ) -> [stl_triangle::TriangleSTL; 4] {
        self.remove_triangle(triangle);

        let new_triangles_points = get_points_triangle_division(triangle);
        let mut vec_new_triangle: Vec<stl_triangle::TriangleSTL> = Vec::new();
        for t_p in new_triangles_points {
            let new_triangle: TriangleSTL =
                stl_triangle::TriangleSTL::new(t_p.0, t_p.1, t_p.2, triangle.normal);
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
