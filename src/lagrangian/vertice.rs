use crate::assert_almost_equal;
use crate::stl_triangle::TriangleSTL;
use crate::utils;
use serde::Serialize;
use std::convert::TryInto;
use std::fmt;

/// Lagranngian node is defined by a position, with normal and area properties
#[derive(Serialize)]
pub struct LagrangianVertice {
    pub pos: utils::Vec3f,
}

impl LagrangianVertice {
    pub fn new(pos: utils::Vec3f) -> LagrangianVertice {
        return LagrangianVertice { pos };
    }

    /// Get lagrangian nodes as bytes in format:
    ///
    /// (pos.x, pos.y, pos.z)
    ///
    /// totalizing 12 bytes
    pub fn get_le_bytes(&self) -> Vec<u8> {
        let vec = self.pos.to_le_bytes_as_f32();

        return vec.to_vec();
    }
}

impl fmt::Display for LagrangianVertice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "pos {}", self.pos);
    }
}

/// Convert STL triangles to lagranagian nodes
pub fn generate_lagrangian_vertices(triangles: &Vec<TriangleSTL>) -> Vec<LagrangianVertice> {
    let v: Vec<LagrangianVertice> = Vec::new();
    return v;
    // return triangles
    //     .iter()
    //     .map(|t| LagrangianVertice::from_triangle(t))
    //     .collect();
}
