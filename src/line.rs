use crate::*;

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

    pub fn slope(&self) -> f32 {
        let &Line { a, b } = self;

        (b.1 - a.1) / (b.0 - a.0)
    }
}

impl Plotable<LinePlot, i64> for Line {
    fn plot(self) -> LinePlot {
        let Line { a, b } = self;

        let (begin, end): (Point<i64>, Point<i64>) = (a.into(), (b - a).into());

        let dx = end.0.abs();
        let dy = end.1.abs();

        let (lead_axis, trail_axis) = if dy > dx {
            (Axis::Y, Axis::X)
        } else {
            (Axis::X, Axis::Y)
        };

        let lead_step = (end.get(&lead_axis) - begin.get(&lead_axis)).signum();
        let trail_step = (end.get(&trail_axis) - begin.get(&trail_axis)).signum();

        let (p, two_d, two_dd) = match lead_axis {
            Axis::X => (2 * dy - dx, 2 * dy, 2 * (dy - dx)),
            Axis::Y => (2 * dx - dy, 2 * dx, 2 * (dx - dy)),
        };

        LinePlot {
            begin,
            end,
            curr: Point(0, 0),
            lead_axis,
            trail_axis,
            lead_step,
            trail_step,
            p,
            two_d,
            two_dd,
        }
    }
}

pub struct LinePlot {
    begin: Point<i64>,
    end: Point<i64>,
    curr: Point<i64>,
    lead_axis: Axis,
    trail_axis: Axis,
    lead_step: i64,
    trail_step: i64,
    p: i64,
    two_d: i64,
    two_dd: i64,
}

impl Iterator for LinePlot {
    type Item = Point<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lead_step.is_negative() {
            if self.curr.get(&self.lead_axis) < self.end.get(&self.lead_axis) {
                return None;
            }
        } else {
            if self.curr.get(&self.lead_axis) > self.end.get(&self.lead_axis) {
                return None;
            }
        };

        let next = Some(Point(
            self.curr.0 + self.begin.0,
            self.curr.1 + self.begin.1,
        ));

        *self.curr.get_mut(&self.lead_axis) += self.lead_step;

        if self.two_d != 0 {
            if self.p < 0 {
                self.p += self.two_d;
            } else {
                *self.curr.get_mut(&self.trail_axis) += self.trail_step;
                self.p += self.two_dd;
            }
        }

        next
    }
}

/* 

Computer Graphics, C Version, 2/e (Page 100)

https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)


∆y = m * ∆x

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
