#[macro_export]
macro_rules! assert_almost_equal {
    ($x:expr, $y:expr, $d:expr) => {
        if (($x - $y).abs() > $d) {
            let error_str = format!("{:.4e} != {:.4e} by more than {:.4e}", $x, $y, $d);
            panic!("{}", error_str);
        };
    };
}

use std::{cmp::Ordering, fmt, hash, ops};

/*
Algebraic Point
*/
#[derive(Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn norm(self) -> f32 {
        return (self.dot(self)).sqrt();
    }

    pub fn dot(self, other: Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn transform(&mut self, factor: f32, offset: Self) {
        *self = *self - offset;
        self.divide(factor)
    }

    pub fn divide(&mut self, denominator: f32) {
        self.x /= denominator;
        self.y /= denominator;
        self.z /= denominator;
    }
}

pub fn almost_equal(x: f32, y: f32, d: f32) -> bool {
    return (x - y < d) || (y - x < d);
}

impl fmt::Display for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z);
    }
}

impl PartialEq for Vec3f {
    fn eq(&self, other: &Self) -> bool {
        let delta = 1e-5f32;
        return almost_equal(self.x, other.x, delta)
            && almost_equal(self.y, other.y, delta)
            && almost_equal(self.z, other.z, delta);
    }
}

impl PartialOrd for Vec3f {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Eq for Vec3f {}

impl Ord for Vec3f {
    fn cmp(&self, other: &Self) -> Ordering {
        let delta = 1e-5f32;
        // check if all numbers are almost equal, if so, points are equal
        if almost_equal(self.x, other.x, delta)
            && almost_equal(self.y, other.y, delta)
            && almost_equal(self.z, other.z, delta)
        {
            return Ordering::Equal;
        }
        // check for dimensions difference, priority is (x, y, z)
        let vals_cmps = [(self.x, other.x), (self.y, other.y), (self.z, other.z)];
        for v in vals_cmps.iter() {
            if !almost_equal(v.0, v.1, delta) {
                if v.0 > v.1 {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        }
        panic!("Invalid comparison between {} and {}. Implementation error", self, other);
    }
}

impl ops::Add for Vec3f {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Self { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z };
    }
}

impl ops::Sub for Vec3f {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl ops::Div for Vec3f {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        return Self { x: self.x / other.x, y: self.y / other.y, z: self.z / other.z };
    }
}

impl ops::Mul for Vec3f {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        return Self { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z };
    }
}

fn truncate_float_to_int(f: f32, n_digits: i32) -> i32 {
    let y = (f * 10f32.powi(n_digits)).round() as i32;
    return y;
}

impl hash::Hash for Vec3f {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // Truncates value up to 5 digits after comma, then hashes it
        let n_digits = 5;
        let (xt, yt, zt) = (
            truncate_float_to_int(self.x, n_digits),
            truncate_float_to_int(self.y, n_digits),
            truncate_float_to_int(self.z, n_digits),
        );
        xt.hash(state);
        yt.hash(state);
        zt.hash(state);
    }
}
