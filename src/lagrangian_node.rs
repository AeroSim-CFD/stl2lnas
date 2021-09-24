use crate::assert_almost_equal;
use crate::stl_triangle::TriangleSTL;
use crate::utils;
use serde::Serialize;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt;

/// Lagranngian node is defined by a position, with normal and area properties
#[derive(Serialize)]
pub struct LagrangianNode {
    pub pos: utils::Vec3f,
    pub normal: utils::Vec3f,
    pub area: f64,
}

pub const LAGRANGIAN_NODE_HEADER: [&str; 7] = [
    "pos_x", "pos_y", "pos_z", "normal_x", "normal_y", "normal_z", "area",
];

impl LagrangianNode {
    pub fn new(pos: utils::Vec3f, normal: utils::Vec3f, area: f64) -> LagrangianNode {
        assert_almost_equal!(normal.norm(), 1f64, 1e-6f64);
        return LagrangianNode { pos, normal, area };
    }

    /// Get lagrangian nodes as bytes in format:
    ///
    /// (pos.x, pos.y, pos.z, normal.x, normal.y, normal.z, area)
    ///
    /// totalizing 28 bytes
    pub fn get_le_bytes(&self) -> [u8; 28] {
        let mut vec = [
            self.pos.to_le_bytes_as_f32(),
            self.normal.to_le_bytes_as_f32(),
        ]
        .concat();
        vec.extend((self.area as f32).to_le_bytes());

        return vec
            .try_into()
            .unwrap_or_else(|v: Vec<u8>| panic!("Expected a Vec of length 12, got {}", v.len()));
    }

    /// Build lagrangian node from triangle
    ///
    /// Position is triangle's middle point, normal and area are the same as triangle's
    pub fn from_triangle(t: &TriangleSTL) -> LagrangianNode {
        let mut triangle_middle_point = t.point0 + t.point1 + t.point2;
        triangle_middle_point.divide(3f64);
        return LagrangianNode::new(triangle_middle_point, t.normal, t.area());
    }
}

impl fmt::Display for LagrangianNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "pos {}, normal {}, area {}",
            self.pos, self.normal, self.area
        );
    }
}

/// Convert STL triangles to lagranagian nodes
pub fn stl2lagrangian(triangles: HashSet<TriangleSTL>) -> Vec<LagrangianNode> {
    return triangles
        .iter()
        .map(|t| LagrangianNode::from_triangle(t))
        .collect();
}
