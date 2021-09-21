use crate::common;
use crate::stl_triangle;
use std::collections::BinaryHeap;

struct IndexedSTL {
    pub points: BinaryHeap<common::Vec3f>,
    pub normals: BinaryHeap<common::Vec3f>,
    pub triangles: Vec<stl_triangle::TriangleSTL>,
}

impl IndexedSTL {
    pub fn new(triangles: Vec<stl_triangle::TriangleSTL>) -> IndexedSTL {
        let mut idx_stl = IndexedSTL {
            points: BinaryHeap::new(),
            normals: BinaryHeap::new(),
            triangles: Vec::new(),
        };
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
    }

    fn add_point(&mut self, point: common::Vec3f) {
        self.points.push(point);
    }

    fn add_normal(&mut self, normal: common::Vec3f) {
        self.normals.push(normal);
    }
}
