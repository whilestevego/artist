use point::*;

pub trait Plotable<I>
where
    I: Iterator<Item = Point>,
{
    fn plot(self) -> I;
}
