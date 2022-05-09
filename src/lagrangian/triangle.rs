use crate::assert_almost_equal;
use crate::lagrangian::vertice::LagrangianVertice;
use crate::stl_triangle::TriangleSTL;
use crate::utils;
use serde::Serialize;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt;

/// Lagranngian node is defined by a position, with normal and area properties
#[derive(Serialize)]
pub struct LagrangianTriangle {
    pub index: utils::Vec3u,
}

impl LagrangianTriangle {
    pub fn new(index: utils::Vec3u) -> LagrangianTriangle {
        return LagrangianTriangle { index };
    }

    pub fn get_le_bytes(&self) -> Vec<u8> {
        let vec = self.index.to_le_bytes_as_u32();

        return vec.to_vec();
    }
}

impl fmt::Display for LagrangianTriangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "index {}", self.index);
    }
}

/// Convert STL triangles to lagranagian nodes
pub fn generate_lagrangian_triangles(
    vertices: &Vec<LagrangianVertice>,
    triangles: &Vec<TriangleSTL>,
) -> Vec<LagrangianTriangle> {
    let v: Vec<LagrangianTriangle> = Vec::new();
    return v;
    // return triangles
    //     .iter()
    //     .map(|t| LagrangianTriangle::from_triangle(t))
    //     .collect();
}
