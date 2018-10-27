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

    pub fn plot(self) -> LinePlot {
        let Line { a, b } = self;
        let slope = self.slope();

        LinePlot {
            base: a,
            curr: Point { x: 0.0, y: 0.0 },
            end: b - a,
            slope,
        }
    }

    pub fn slope(&self) -> f64 {
        let &Line { a, b } = self;

        (b.y - a.y) / (b.x - a.x)
    }
}

#[derive(Debug)]
pub struct LinePlot {
    base: Point,
    curr: Point,
    end: Point,
    slope: f64,
}

impl LinePlot {
    pub fn merge(&mut self, line_plot: LinePlot) -> &mut Self {
        self.base = line_plot.base;
        self.curr = line_plot.curr;
        self.end = line_plot.end;
        self.slope = line_plot.slope;

        self
    }
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
