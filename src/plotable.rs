use vector::*;

pub trait Plotable<I>
where
    I: Iterator<Item = Vector>,
{
    fn plot(self) -> I;
}
