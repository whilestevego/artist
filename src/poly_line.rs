use line::*;
use plotable::*;
use std::collections::VecDeque;
use vector::*;

#[derive(Debug)]
pub struct PolyLine {
    vectors: VecDeque<Vector>,
}

impl PolyLine {
    pub fn new(vectors: Vec<impl Into<Vector>>) -> Self {
        Self {
            vectors: vectors.into_iter().map(|p| p.into()).collect(),
        }
    }

    pub fn to(mut self, point: impl Into<Vector>) -> Self {
        self.vectors.push_back(point.into());
        self
    }
}

impl Plotable<PolyLinePlot> for PolyLine {
    fn plot(self) -> PolyLinePlot {
        let PolyLine { mut vectors } = self;
        let a = vectors.pop_front().unwrap();
        let b = *vectors.front().unwrap();

        PolyLinePlot {
            line_plot: Line::new(a, b).plot(),
            rem_points: vectors,
        }
    }
}

impl<T: Into<Vector>> From<Vec<T>> for PolyLine {
    fn from(vectors: Vec<T>) -> Self {
        Self::new(vectors)
    }
}

pub struct PolyLinePlot {
    line_plot: LinePlot,
    rem_points: VecDeque<Vector>,
}

impl Iterator for PolyLinePlot {
    type Item = Vector;

    fn next(&mut self) -> Option<Self::Item> {
        let PolyLinePlot {
            line_plot,
            rem_points,
        } = self;

        line_plot.next().or_else(|| {
            let next_a = rem_points.pop_front().unwrap();

            if let Some(&next_b) = rem_points.front() {
                *line_plot = Line::new(next_a, next_b).plot();
                line_plot.next();
                line_plot.next()
            } else {
                None
            }
        })
    }
}
