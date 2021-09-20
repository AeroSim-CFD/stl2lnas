#[macro_export]
macro_rules! assert_almost_equal {
    ($x:expr, $y:expr, $d:expr) => {
        if (($x - $y).abs() > $d) {
            let error_str = format!("{:.4e} != {:.4e} by more than {:.4e}", $x, $y, $d);
            panic!("{}", error_str);
        };
    };
}

use std::fmt;

/*
Algebraic Point
*/
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn norm(&self) -> f32 {
        return (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt();
    }
}

pub fn almost_equal(x: f32, y: f32, d: f32) -> bool {
    return (x - y < d) || (y - x < d);
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z);
    }
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        let delta = 1e-5f32;
        return almost_equal(self.x, other.x, delta)
            && almost_equal(self.y, other.y, delta)
            && almost_equal(self.z, other.z, delta);
    }
}

impl Eq for Point3D {}
