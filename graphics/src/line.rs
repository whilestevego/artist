use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Line {
    start: Vector,
    end: Vector,
}

impl Line {
    pub fn new(start: impl Into<Vector>, end: impl Into<Vector>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    pub fn slope(&self) -> f32 {
        let &Line { start, end } = self;

        (end.1 - start.1) / (end.0 - start.0)
    }
}

/// Implementation of Bresenham's Line Drawing Algorithm
impl Plotable for Line {
    fn plot(self) -> Plot {
        let Line { start, end } = self;

        let (first, last): (Point<i32>, Point<i32>) = (start.into(), (end - start).into());

        let dx = last.0.abs();
        let dy = last.1.abs();

        let (lead_axis, trail_axis) = if dy > dx {
            (Axis::Y, Axis::X)
        } else {
            (Axis::X, Axis::Y)
        };

        let lead_step = (last.get(&lead_axis) - first.get(&lead_axis)).signum();
        let trail_step = (last.get(&trail_axis) - first.get(&trail_axis)).signum();

        let (p, two_d, two_dd) = match lead_axis {
            Axis::X => (2 * dy - dx, 2 * dy, 2 * (dy - dx)),
            Axis::Y => (2 * dx - dy, 2 * dx, 2 * (dx - dy)),
        };

        Box::new(LinePlot {
            first,
            last,
            curr: Point(0, 0),
            lead_axis,
            trail_axis,
            lead_step,
            trail_step,
            p,
            two_d,
            two_dd,
        })
    }
}

pub struct LinePlot {
    first: Point<i32>,
    last: Point<i32>,
    curr: Point<i32>,
    lead_axis: Axis,
    trail_axis: Axis,
    lead_step: i32,
    trail_step: i32,
    p: i32,
    two_d: i32,
    two_dd: i32,
}

impl Iterator for LinePlot {
    type Item = Point<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let LinePlot {
            ref first,
            ref mut curr,
            ref mut p,
            ref mut two_d,
            ref mut two_dd,
            ref trail_axis,
            ref trail_step,
            ref lead_step,
            ref lead_axis,
            ref last,
        } = self;

        if lead_step.is_negative() {
            if curr.get(lead_axis) < last.get(lead_axis) {
                return None;
            }
        } else if curr.get(lead_axis) > last.get(lead_axis) {
            return None;
        };

        let next = Some(Point(curr.0 + first.0, curr.1 + first.1));

        *curr.get_mut(lead_axis) += lead_step;

        if *two_d != 0 {
            if p.is_negative() {
                *p += *two_d;
            } else {
                *curr.get_mut(trail_axis) += trail_step;
                *p += *two_dd;
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
