use std::cmp::Ordering;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn direction(&self) -> Self {
        let magnitude = self.magnitude();

        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

    pub fn round(self) -> Self {
        self.x.round();
        self.y.round();
        self
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x
            .partial_cmp(&other.x)
            .and_then(|ord| {
                if ord == Ordering::Equal {
                    self.y.partial_cmp(&other.y)
                } else {
                    Some(ord)
                }
            }).expect("Could not compare values")
    }
}

impl PartialEq<(f64, f64)> for Point {
    fn eq(&self, other: &(f64, f64)) -> bool {
        self.x.eq(&other.0) && self.y.eq(&other.1)
    }
}

impl PartialEq<Point> for (f64, f64) {
    fn eq(&self, other: &Point) -> bool {
        other.x.eq(&self.0) && other.y.eq(&self.1)
    }
}

impl Eq for Point {}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
        }
    }
}
