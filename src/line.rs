use point::*;

#[derive(Default, Debug)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(begin: impl Into<Point>, end: impl Into<Point>) -> Line {
        Line {
            a: begin.into(),
            b: end.into(),
        }
    }

    pub fn points(self) -> LineIter {
        let ptr = self.a;
        let slope = self.slope();

        LineIter {
            line: self,
            slope,
            ptr,
        }
    }

    pub fn slope(&self) -> f64 {
        let (delta_x, delta_y) = self.delta();

        (delta_y / delta_x).round()
    }

    fn delta(&self) -> (f64, f64) {
        let &Line {
            a: Point { x: xa, y: ya },
            b: Point { x: xb, y: yb },
        } = self;

        (xb - xa, yb - ya)
    }
}

#[derive(Debug)]
pub struct LineIter {
    line: Line,
    ptr: Point,
    slope: f64,
}

impl Iterator for LineIter {
    type Item = Point;

    // Generates all points to draw a line following Bresenham's algorithm using DDA
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    // https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)
    fn next(&mut self) -> Option<Self::Item> {
        let &mut LineIter {
            ref line,
            slope,
            ptr: curr,
        } = self;
        let Line { a, b } = line;

        // Exit if curr is past b in any direction
        if curr.x.abs() > b.x.abs() || curr.y.abs() > b.y.abs() {
            return None;
        }

        // Calculate and assign next self.ptr
        self.ptr = if slope.abs() > 1.0 {
            let step = (b.y - a.y).signum();
            Point::from(((1.0 / slope * step) + curr.x, curr.y + step))
        } else {
            let step = (b.x - a.x).signum();
            Point::from((curr.x + step, (slope * step) + curr.y))
        };

        Some(curr)
    }
}
