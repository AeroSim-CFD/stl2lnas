use crate::assert_almost_equal;
use crate::common;
use std::fmt;

pub struct LagrangianNode {
    pub pos: common::Vec3f,
    pub normal: common::Vec3f,
    pub area: f64,
}

impl LagrangianNode{
    pub fn new(pos: common::Vec3f, normal: common::Vec3f, area: f64) -> LagrangianNode{
        assert_almost_equal!(normal.norm(), 1f64, 1e-6f64);
        return LagrangianNode{pos, normal, area};
    }
}

impl fmt::Display for LagrangianNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "pos {}, normal {}, area {}", self.pos, self.normal, self.area);
    }
}