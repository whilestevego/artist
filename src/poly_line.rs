use line::*;
use plotable::*;
use point::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct PolyLine {
    points: VecDeque<Point>,
}

impl PolyLine {
    pub fn new(points: Vec<impl Into<Point>>) -> Self {
        Self {
            points: points.into_iter().map(|p| p.into()).collect(),
        }
    }

    pub fn to(mut self, point: impl Into<Point>) -> Self {
        self.points.push_back(point.into());
        self
    }
}

impl Plotable<PolyLinePlot> for PolyLine {
    fn plot(self) -> PolyLinePlot {
        let PolyLine { mut points } = self;
        let a = points.pop_front().unwrap();
        let b = *points.front().unwrap();

        PolyLinePlot {
            line_plot: Line::new(a, b).plot(),
            rem_points: points,
        }
    }
}

impl<T: Into<Point>> From<Vec<T>> for PolyLine {
    fn from(points: Vec<T>) -> Self {
        Self::new(points)
    }
}

pub struct PolyLinePlot {
    line_plot: LinePlot,
    rem_points: VecDeque<Point>,
}

impl Iterator for PolyLinePlot {
    type Item = Point;

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
