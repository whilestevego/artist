use plotable::*;
use vector::*;

#[derive(Clone, Default, Debug)]
pub struct Line {
    a: Vector,
    b: Vector,
}

impl Line {
    pub fn new(begin: impl Into<Vector>, end: impl Into<Vector>) -> Line {
        Self {
            a: begin.into(),
            b: end.into(),
        }
    }

    pub fn slope(&self) -> f64 {
        let &Line { a, b } = self;

        (b.1 - a.1) / (b.0 - a.0)
    }
}

impl Plotable<LinePlot> for Line {
    fn plot(self) -> LinePlot {
        let Line { a, b } = self;
        let slope = self.slope();

        LinePlot {
            base: a,
            curr: Vector(0.0, 0.0),
            end: b - a,
            slope,
        }
    }
}

pub struct LinePlot {
    base: Vector,
    curr: Vector,
    end: Vector,
    slope: f64,
}

impl Iterator for LinePlot {
    type Item = Vector;

    // TODO: Review algorithm and compare to Bresenham's
    fn next(&mut self) -> Option<Self::Item> {
        let &mut LinePlot {
            base,
            curr,
            end,
            slope,
        } = self;

        // Exit if curr is past b in any direction
        if curr.0.abs() > end.0.abs() || curr.1.abs() > end.1.abs() {
            return None;
        }

        // Calculate and assign next self.ptr
        self.curr = if slope.abs() > 1.0 {
            let step = end.1.signum();
            Vector((1.0 / slope * step) + curr.0, curr.1 + step)
        } else {
            let step = end.0.signum();
            Vector(curr.0 + step, (slope * step) + curr.1)
        };

        Some(curr + base)
    }
}

/* 

Computer Graphics, C Version, 2/e (Page 100)

https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)

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
