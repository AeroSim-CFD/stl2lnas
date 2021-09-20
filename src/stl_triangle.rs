use crate::assert_almost_equal;
use crate::common;
use std::fmt;

pub struct TriangleSTL {
    pub point0: common::Point3D,
    pub point1: common::Point3D,
    pub point2: common::Point3D,
    pub normal: common::Point3D,
}

impl TriangleSTL {
    pub fn new(
        point0: common::Point3D,
        point1: common::Point3D,
        point2: common::Point3D,
        normal: common::Point3D,
    ) -> TriangleSTL {
        assert_almost_equal!(&normal.norm(), 1.0, 1e-5f32);
        return TriangleSTL {
            point0,
            point1,
            point2,
            normal,
        };
    }
}

impl fmt::Display for TriangleSTL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "(p0: {}, p1: {}, p2: {}, normal: {})",
            self.point0, self.point1, self.point2, self.normal
        );
    }
}
