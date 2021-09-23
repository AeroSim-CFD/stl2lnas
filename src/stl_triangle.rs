use crate::assert_almost_equal;
use crate::common;
use std::fmt;

#[derive(Clone, Copy, Hash)]
pub struct TriangleSTL {
    pub point0: common::Vec3f,
    pub point1: common::Vec3f,
    pub point2: common::Vec3f,
    pub normal: common::Vec3f,
}

impl TriangleSTL {
    pub fn new(
        point0: common::Vec3f,
        point1: common::Vec3f,
        point2: common::Vec3f,
        normal: common::Vec3f,
    ) -> TriangleSTL {
        assert_almost_equal!(&normal.norm(), 1.0, 1e-5f64);
        return TriangleSTL {
            point0,
            point1,
            point2,
            normal,
        };
    }

    pub fn normalize(&mut self, factor: f64, offset: common::Vec3f) {
        self.point0.transform(factor, offset);
        self.point1.transform(factor, offset);
        self.point2.transform(factor, offset);
    }

    pub fn area(self) -> f64 {
        let tr_vec1 = self.point0 - self.point1;
        let tr_vec2 = self.point2 - self.point1;
        let cross_prod: common::Vec3f = tr_vec1.cross(tr_vec2);
        let area = cross_prod.norm();
        return area;
    }
}

impl PartialEq for TriangleSTL {
    fn eq(&self, other: &Self) -> bool {
        return self.point0 == other.point0
            && self.point1 == other.point1
            && self.point2 == other.point2
            && self.normal == other.normal;
    }
}

impl Eq for TriangleSTL {}

impl fmt::Display for TriangleSTL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "(p0: {}, p1: {}, p2: {}, normal: {})",
            self.point0, self.point1, self.point2, self.normal
        );
    }
}

fn get_factor_offset(
    min_vals: common::Vec3f,
    max_vals: common::Vec3f,
    total_dist_x: f64,
) -> (f64, common::Vec3f) {
    // Params are: the minimal value, and the difference between min_max for x
    // These can be used to normalize points
    let mul_factor = total_dist_x / (max_vals.x - min_vals.x); // normalize between 0 and total_dist
    let offset = common::Vec3f {
        x: min_vals.x,
        y: min_vals.y,
        z: min_vals.z,
    };
    return (mul_factor, offset);
}

fn get_triangles_min_max(triangles: &Vec<TriangleSTL>) -> (common::Vec3f, common::Vec3f) {
    let mut min_vals = common::Vec3f {
        x: f64::MAX,
        y: f64::MAX,
        z: f64::MAX,
    };
    let mut max_vals = common::Vec3f {
        x: f64::MIN,
        y: f64::MIN,
        z: f64::MIN,
    };
    for t in triangles {
        for p in [t.point0, t.point1, t.point2] {
            if p.x < min_vals.x {
                min_vals.x = p.x;
            } else if p.x > max_vals.x {
                max_vals.x = p.x;
            }
            if p.y < min_vals.y {
                min_vals.y = p.y;
            } else if p.y > max_vals.y {
                max_vals.y = p.y;
            }
            if p.z < min_vals.z {
                min_vals.z = p.z;
            } else if p.z > max_vals.z {
                max_vals.z = p.z;
            }
        }
    }
    return (min_vals, max_vals);
}

pub fn normalize_triangles(triangles: &Vec<TriangleSTL>, total_dist_x: f64) -> Vec<TriangleSTL> {
    let (min_vals, max_vals) = get_triangles_min_max(&triangles);
    let (mul_factor, offset) = get_factor_offset(min_vals, max_vals, total_dist_x);
    let mut normalized_triangles: Vec<TriangleSTL> = Vec::new();
    for t in triangles.iter() {
        let mut normalized_t = Clone::clone(t);
        normalized_t.normalize(mul_factor, offset);
        normalized_triangles.push(normalized_t);
    }
    return normalized_triangles;
}
