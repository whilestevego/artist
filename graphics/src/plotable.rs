use crate::*;

pub type Plot = Box<dyn Iterator<Item = Point<i32>>>;

pub trait Plotable {
    fn plot(self) -> Plot;
}
