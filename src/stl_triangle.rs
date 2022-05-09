use crate::assert_almost_equal;
use crate::utils;
use std::cmp;
use std::fmt;

#[derive(Clone, Copy, Hash)]
pub struct TriangleSTL {
    pub point0: utils::Vec3f,
    pub point1: utils::Vec3f,
    pub point2: utils::Vec3f,
    pub normal: utils::Vec3f,
}

impl TriangleSTL {
    pub fn new(
        point0: utils::Vec3f,
        point1: utils::Vec3f,
        point2: utils::Vec3f,
        normal: utils::Vec3f,
    ) -> TriangleSTL {
        assert_almost_equal!(&normal.norm(), 1.0, 1e-5f32);
        return TriangleSTL {
            point0,
            point1,
            point2,
            normal,
        };
    }

    pub fn normalize(&mut self, factor: f32, offset: utils::Vec3f) {
        self.point0.transform(factor, offset);
        self.point1.transform(factor, offset);
        self.point2.transform(factor, offset);
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
    min_vals: utils::Vec3f,
    max_vals: utils::Vec3f,
    total_dist_x: f32,
) -> (f32, utils::Vec3f) {
    // Params are: the minimal value, and the difference between min_max for x
    // These can be used to normalize points
    let mul_factor = total_dist_x / (max_vals.x - min_vals.x); // normalize between 0 and total_dist
    let offset = utils::Vec3f {
        x: min_vals.x,
        y: min_vals.y,
        z: min_vals.z,
    };
    return (mul_factor, offset);
}

fn get_triangles_min_max(triangles: &Vec<TriangleSTL>) -> (utils::Vec3f, utils::Vec3f) {
    let mut min_vals = utils::Vec3f {
        x: f32::MAX,
        y: f32::MAX,
        z: f32::MAX,
    };
    let mut max_vals = utils::Vec3f {
        x: f32::MIN,
        y: f32::MIN,
        z: f32::MIN,
    };
    for t in triangles {
        for p in [&t.point0, &t.point1, &t.point2] {
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

pub fn normalize_triangles(triangles: &Vec<TriangleSTL>, total_dist_x: f32) -> Vec<TriangleSTL> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stl_reader::read_stl;
    use crate::utils::almost_equal;

    fn check_normalization(norm_triangles: Vec<TriangleSTL>, norm_dist: f32) {
        for t in norm_triangles.iter() {
            for p in [&t.point0, &t.point1, &t.point2] {
                if (p.x < 0f32 || p.x > norm_dist) {
                    panic!(format!("Point {} x is not between 0 and {}", p, norm_dist));
                }
            }
        }
        let max_x = norm_triangles
            .iter()
            .flat_map(|t: &TriangleSTL| Vec::from([t.point0.x, t.point1.x, t.point2.x]))
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        let min_x = norm_triangles
            .iter()
            .flat_map(|t: &TriangleSTL| Vec::from([t.point0.x, t.point1.x, t.point2.x]))
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();

        if !almost_equal(max_x, norm_dist) {
            panic!("Max is not same as {}", norm_dist);
        }
        if !almost_equal(min_x, 0f32) {
            panic!("Min is not same as 0");
        }
    }

    #[test]
    fn normalizes_stl_cube() {
        let filename = String::from("examples/stl/cube.stl");
        let norm_dist: f32 = 3.5;
        let triangles = read_stl(&filename);
        let norm_triangles = normalize_triangles(&triangles, norm_dist);
        check_normalization(norm_triangles, norm_dist);
    }

    #[test]
    fn normalizes_stl_terrain() {
        let filename = String::from("examples/stl/terrain.stl");
        let norm_dist: f32 = 15.0;
        let triangles = read_stl(&filename);
        let norm_triangles = normalize_triangles(&triangles, norm_dist);
        check_normalization(norm_triangles, norm_dist);
    }
}
