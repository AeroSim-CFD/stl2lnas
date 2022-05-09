use crate::lagrangian::vertice::LagrangianVertice;
use crate::stl::triangle::TriangleSTL;
use crate::utils;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;

/// Lagranngian node is defined by a position, with normal and area properties
#[derive(Serialize, Clone, Copy)]
pub struct LagrangianTriangle {
    pub p0: LagrangianVertice,
    pub p1: LagrangianVertice,
    pub p2: LagrangianVertice,
}

impl LagrangianTriangle {
    pub fn new(
        p0: LagrangianVertice,
        p1: LagrangianVertice,
        p2: LagrangianVertice,
    ) -> LagrangianTriangle {
        // TODO: check for normal when creating
        return LagrangianTriangle { p0, p1, p2 };
    }

    pub fn get_indexes(&self, triangles: &HashMap<LagrangianVertice, usize>) -> Vec<usize> {
        let idx0 = triangles.get(&self.p0).unwrap().clone();
        let idx1 = triangles.get(&self.p1).unwrap().clone();
        let idx2 = triangles.get(&self.p2).unwrap().clone();
        return [idx0, idx1, idx2].to_vec();
    }

    pub fn get_le_bytes(&self, triangles: &HashMap<LagrangianVertice, usize>) -> Vec<u8> {
        let vec = self.get_indexes(triangles);
        let vec_byte: Vec<u8> = vec.iter().flat_map(|x| x.to_le_bytes()).collect();
        return vec_byte;
    }
}

impl fmt::Display for LagrangianTriangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "p0 {} p1 {} p2 {}", self.p0, self.p1, self.p2);
    }
}

/// Convert STL triangles to lagranagian nodes
pub fn generate_lagrangian_triangles(
    vertices: &HashMap<LagrangianVertice, usize>,
    triangles: &Vec<TriangleSTL>,
) -> Vec<LagrangianTriangle> {
    let mut lagrangian_triangles: Vec<LagrangianTriangle> = Vec::new();
    for t in triangles.iter() {
        let lagr_p0 = LagrangianVertice::new(t.point0.clone());
        let lagr_p1 = LagrangianVertice::new(t.point1.clone());
        let lagr_p2 = LagrangianVertice::new(t.point2.clone());
        let lagr_tri = LagrangianTriangle::new(lagr_p0, lagr_p1, lagr_p2);
        lagrangian_triangles.push(lagr_tri);
    }
    return lagrangian_triangles;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lagrangian::vertice::generate_lagrangian_vertices;
    use crate::stl::reader::read_stl;

    #[test]
    fn check_triangles_stl_cube() {
        let filename = String::from("examples/stl/cube.stl");
        let triangles = read_stl(&filename);
        let lagr_vertices = generate_lagrangian_vertices(&triangles);
        let lagr_triangles = generate_lagrangian_triangles(&lagr_vertices, &triangles);

        assert_eq!(lagr_triangles.len(), triangles.len());
    }

    #[test]
    fn check_triangles_stl_terrain() {
        let filename = String::from("examples/stl/terrain.stl");
        let triangles = read_stl(&filename);
        let lagr_vertices = generate_lagrangian_vertices(&triangles);
        let lagr_triangles = generate_lagrangian_triangles(&lagr_vertices, &triangles);

        assert_eq!(lagr_triangles.len(), triangles.len());
    }
}
