use crate::*;
use num::Integer;

#[derive(Clone, Copy, Eq, Debug, Default, Ord, PartialEq, PartialOrd)]
pub struct Point<T: Integer>(pub T, pub T);

impl<T: Integer + Copy> Point<T> {
    pub fn get(&self, axis: &Axis) -> T {
        match axis {
            Axis::X => self.0,
            Axis::Y => self.1,
        }
    }

    pub fn get_mut(&mut self, axis: &Axis) -> &mut T {
        match axis {
            Axis::X => &mut self.0,
            Axis::Y => &mut self.1,
        }
    }
}

impl<T: Integer> PartialEq<(T, T)> for Point<T> {
    fn eq(&self, other: &(T, T)) -> bool {
        self.0.eq(&other.0) && self.1.eq(&other.1)
    }
}

macro_rules! impl_point {
    ($T:ty) => {
        impl From<Vector> for Point<$T> {
            fn from(v: Vector) -> Self {
                Point(v.0.round() as $T, v.1.round() as $T)
            }
        }

        impl PartialEq<Point<$T>> for ($T, $T) {
            fn eq(&self, other: &Point<$T>) -> bool {
                other.0.eq(&self.0) && other.1.eq(&self.1)
            }
        }
    };
}

impl_point!(i8);
impl_point!(i16);
impl_point!(i32);
impl_point!(i64);
