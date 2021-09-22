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

const PREC_DIGITS: i32 = 6i32;

fn truncate_float_to_int(f: f64, n_digits: i32) -> i64 {
    let y = (f * 10f64.powi(n_digits)).round() as i64;
    return y;
}

/*
Algebraic Point
*/
#[derive(Clone, Copy)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3f {
    pub fn norm(self) -> f64 {
        return (self.dot(self)).sqrt();
    }

    pub fn normalize(&mut self){
        self.divide(self.norm());
    }

    pub fn dot(self, other: Vec3f) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn transform(&mut self, factor: f64, offset: Self) {
        *self = *self - offset;
        self.divide(factor)
    }

    pub fn divide(&mut self, denominator: f64) {
        self.x /= denominator;
        self.y /= denominator;
        self.z /= denominator;
    }

    pub fn multiply(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    pub fn cross(self, other: Self) -> Self {
        return Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        };
    }

    pub fn abs(self) -> Self {
        let mut x = self.x;
        if x < 0f64 {
            x = -x;
        }
        let mut y = self.y;
        if y < 0f64 {
            y = -y;
        }
        let mut z = self.z;
        if z < 0f64 {
            z = -z;
        }
        return Vec3f { x, y, z };
    }
}

pub fn almost_equal(x: f64, y: f64) -> bool {
    return truncate_float_to_int(x, PREC_DIGITS) == truncate_float_to_int(y, PREC_DIGITS);
}

impl fmt::Display for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z);
    }
}

impl PartialEq for Vec3f {
    fn eq(&self, other: &Self) -> bool {
        return almost_equal(self.x, other.x)
            && almost_equal(self.y, other.y)
            && almost_equal(self.z, other.z);
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
        // check if all numbers are almost equal, if so, points are equal
        if almost_equal(self.x, other.x)
            && almost_equal(self.y, other.y)
            && almost_equal(self.z, other.z)
        {
            return Ordering::Equal;
        }
        // check for dimensions difference, priority is (x, y, z)
        let vals_cmps = [(self.x, other.x), (self.y, other.y), (self.z, other.z)];
        for v in vals_cmps.iter() {
            if !almost_equal(v.0, v.1) {
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

impl ops::AddAssign for Vec3f {
    fn add_assign(&mut self, other: Self) {
        *self = *self+other;
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

impl hash::Hash for Vec3f {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // Truncates value up to 5 digits after comma, then hashes it
        let n_digits = PREC_DIGITS;
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
