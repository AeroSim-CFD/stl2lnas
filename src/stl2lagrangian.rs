use crate::common::Vec3f;
use crate::lagrangian_node::LagrangianNode;
use crate::stl_divider::DividerSTL;
use crate::stl_triangle::TriangleSTL;
use std::collections::HashMap;
use std::vec;

struct NodeTriangles {
    pub pos: Vec3f,
    pub triangles: Vec<TriangleSTL>,
}

impl NodeTriangles {
    pub fn lagrangian_node(self) -> LagrangianNode {
        if self.triangles.len() == 0usize {
            panic!(
                "Node at {} is associated with no triangle, unable to generate lagrangian node.",
                self.pos
            );
        }
        let triangles_areas: Vec<f64> = self.triangles.iter().map(|t| t.area()).collect();

        let mut node_normal = Vec3f {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        };
        let mut avg_areas = 0f64;

        for (i, t) in self.triangles.iter().enumerate() {
            // Sum triangle area
            avg_areas += t.area() / (self.triangles.len() as f64);
            // Normal is weighted with its triangle area
            let mut normal_sum = t.normal;
            normal_sum.multiply(triangles_areas[i]);
            node_normal += normal_sum;
        }
        // Normalize normal
        node_normal.normalize();

        return LagrangianNode::new(self.pos, node_normal, avg_areas);
    }
}

fn add_triange_to_hash_map(t_nodes_hash_map: &mut HashMap<Vec3f, NodeTriangles>, t: &TriangleSTL) {
    for p in [t.point0, t.point1, t.point2] {
        t_nodes_hash_map
            .entry(p)
            .and_modify(|tn| tn.triangles.push(t.clone()))
            .or_insert(NodeTriangles {
                pos: p,
                triangles: vec![t.clone()],
            });
    }
}

fn node_triangles_from_div_stl(div_stl: &DividerSTL) -> Vec<NodeTriangles> {
    let mut t_nodes_hash_map: HashMap<Vec3f, NodeTriangles> = HashMap::new();
    for t in div_stl.triangles.iter() {
        add_triange_to_hash_map(&mut t_nodes_hash_map, t);
    }

    return t_nodes_hash_map.into_iter().map(|(_k, nt)| nt).collect();
}

pub fn stl2lagrangian(div_stl: &DividerSTL) -> Vec<LagrangianNode> {
    let node_triangles = node_triangles_from_div_stl(div_stl);

    return node_triangles
        .into_iter()
        .map(|nt| nt.lagrangian_node())
        .collect();
}
