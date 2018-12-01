use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    /// The scalar porpotion of the vector or its length.
    ///
    /// ```
    /// extern crate graphics;
    /// use graphics::*;
    ///
    /// assert_eq!(Vector(1.0, 1.0).magnitude(), 1.4142135623730951);
    /// assert_eq!(Vector(3.0, 4.0).magnitude(), 5.0);
    /// ```
    pub fn magnitude(&self) -> f32 {
        (self.0.powf(2.0) + self.1.powf(2.0)).sqrt()
    }

    /// A unit length vector representing the vector's direction.
    ///
    /// ```
    /// extern crate graphics;
    /// use graphics::*;
    ///
    /// assert_eq!(Vector(1.0, 1.0).direction(), Vector(0.7071067811865475, 0.7071067811865475));
    /// assert_eq!(Vector(3.0, 4.0).direction(), Vector(0.6, 0.8));
    /// ```
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

    /// Implements `+` for Vector
    ///
    /// ```
    /// extern crate graphics;
    /// use graphics::*;
    ///
    /// assert_eq!(Vector(1.0, 1.0) + Vector(1.0, 1.0), Vector(2.0, 2.0));
    /// assert_eq!(Vector(3.0, 4.0) + Vector(7.0, -8.0), Vector(10.0, -4.0));
    /// ```
    fn add(self, other: Self) -> Self {
        Vector(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vector {
    type Output = Self;

    /// Implements `-` for Vector
    ///
    /// ```
    /// extern crate graphics;
    /// use graphics::*;
    ///
    /// assert_eq!(Vector(1.0, 1.0) - Vector(1.0, 1.0), Vector(0.0, 0.0));
    /// assert_eq!(Vector(3.0, 4.0) - Vector(7.0, -8.0), Vector(-4.0, 12.0));
    /// ```
    fn sub(self, other: Self) -> Self {
        Vector(self.0 - other.0, self.1 - other.1)
    }
}

// ORDERING

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .and_then(|ord| match ord {
                Ordering::Equal => self.1.partial_cmp(&other.1),
                _ => Some(ord),
            }).expect("Could not compare values")
    }
}

// EQUALITY

impl PartialEq<(f32, f32)> for Vector {
    fn eq(&self, other: &(f32, f32)) -> bool {
        self.0.eq(&other.0) && self.1.eq(&other.1)
    }
}

impl PartialEq<Vector> for (f32, f32) {
    fn eq(&self, other: &Vector) -> bool {
        other.0.eq(&self.0) && other.1.eq(&self.1)
    }
}

impl Eq for Vector {}

impl From<(f32, f32)> for Vector {
    fn from((x, y): (f32, f32)) -> Self {
        Vector(x, y)
    }
}
