use crate::assert_almost_equal;
use crate::utils;
use serde::Serialize;
use std::convert::TryInto;
use std::fmt;

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
