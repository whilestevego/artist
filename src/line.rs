use plotable::*;
use point::*;

#[derive(Clone, Default, Debug)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(begin: impl Into<Point>, end: impl Into<Point>) -> Line {
        Self {
            a: begin.into(),
            b: end.into(),
        }
    }

    pub fn slope(&self) -> f64 {
        let &Line { a, b } = self;

        (b.y - a.y) / (b.x - a.x)
    }
}

impl Plotable<LinePlot> for Line {
    fn plot(self) -> LinePlot {
        let Line { a, b } = self;
        let slope = self.slope();

        LinePlot {
            base: a,
            curr: Point { x: 0.0, y: 0.0 },
            end: b - a,
            slope,
        }
    }
}

pub struct LinePlot {
    base: Point,
    curr: Point,
    end: Point,
    slope: f64,
}

impl Iterator for LinePlot {
    type Item = Point;

    // Generates all points to draw a line following Bresenham's algorithm using DDA
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    // https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)
    fn next(&mut self) -> Option<Self::Item> {
        let &mut LinePlot {
            base,
            curr,
            end,
            slope,
        } = self;

        // Exit if curr is past b in any direction
        if curr.x.abs() > end.x.abs() || curr.y.abs() > end.y.abs() {
            return None;
        }

        // Calculate and assign next self.ptr
        self.curr = if slope.abs() > 1.0 {
            let step = end.y.signum();
            Point::new((1.0 / slope * step) + curr.x, curr.y + step)
        } else {
            let step = end.x.signum();
            Point::new(curr.x + step, (slope * step) + curr.y)
        };

        Some(curr + base)
    }
}

/* (Page 100)

Bresenham's Line-Drawing Algorithm for |m| < 1

1.  Input the twoline endpoints and store the left endpoint in (xo,yo)
2.  Load (xo,yo) into the frame buffer; that is plot the first point.

3.  Calculate constants ∆x, ∆y, 2∆y, and 2∆y - 2∆r, and obtain the starting
    value for the decision parameter as po = 2∆y - ∆X.

4.  At each xk along the line, starting at k = 0, perform the following test:
    If pk < 0, the next point to plot is (xk + 1, yk) and
        pk+1 = pk + 2∆y

    Otherwise, the next point to plot is (xk + 1, yk + 1) and
        pk+1 = pk + 2∆y - 2∆x

5.  Repeat step 4 ∆x times.

*/
