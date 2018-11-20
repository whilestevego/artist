use crate::*;
use num::Integer;

pub trait Plotable<I, T>
where
    I: Iterator<Item = Point<T>>,
    T: Integer,
{
    fn plot(self) -> I;
}
