use std::cmp::Ordering;
use std::ops::{Add, Sub};

//TODO: Use num create to make Vectors generic over numbers
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector(pub f64, pub f64);

impl Vector {
    pub fn magnitude(&self) -> f64 {
        (self.0.powf(2.0) + self.1.powf(2.0)).sqrt()
    }

    pub fn direction(&self) -> Self {
        let magnitude = self.magnitude();

        Vector(self.0 / magnitude, self.1 / magnitude)
    }

    pub fn round(self) -> Self {
        self.0.round();
        self.1.round();
        self
    }
}

// ARITHMETIC OPERATORS

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector(self.0 - other.0, self.1 - other.1)
    }
}

// ORDERING

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .and_then(|ord| {
                if ord == Ordering::Equal {
                    self.1.partial_cmp(&other.1)
                } else {
                    Some(ord)
                }
            }).expect("Could not compare values")
    }
}

// EQUALITY

impl PartialEq<(f64, f64)> for Vector {
    fn eq(&self, other: &(f64, f64)) -> bool {
        self.0.eq(&other.0) && self.1.eq(&other.1)
    }
}

impl PartialEq<Vector> for (f64, f64) {
    fn eq(&self, other: &Vector) -> bool {
        other.0.eq(&self.0) && other.1.eq(&self.1)
    }
}

impl Eq for Vector {}

impl From<(f64, f64)> for Vector {
    fn from((x, y): (f64, f64)) -> Self {
        Vector(x, y)
    }
}

impl From<(f32, f32)> for Vector {
    fn from((x, y): (f32, f32)) -> Self {
        Vector(x as f64, y as f64)
    }
}
