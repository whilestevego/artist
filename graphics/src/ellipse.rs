use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Ellipse {
    center: Vector,
    radius: (f32, f32),
}

impl Ellipse {
    pub fn new(center: impl Into<Vector>, radius: (f32, f32)) -> Self {
        Self {
            center: center.into(),
            radius,
        }
    }
}

// Ellipse Quadrants
//
//           +Y (y,x)
//      \  7  |  0  /
//       \    |    /
//        \   |   /   (x, y)
//     6   \  |  /   1
//          \ | /
// -X ----------------- +X
//          / | \
//     5   /  |  \   2
//        /   |   \
//       /    |    \
//      /  4  |  3  \
//            -Y

/// Implements Midpoint Ellipse Algorithm
///
/// Algorith Reference:
/// A Rasterizing Algorithm for Drawing Curves
/// Multimedia und Softwareentwicklung
/// Technikum-Wien
/// Alois Zingl
/// Wien, 2012
impl Plotable for Ellipse {
    fn plot(self) -> Plot {
        let Ellipse { center, radius } = self;
        let e2 = radius.1 * radius.1;
        let err = -radius.0 * (2.0 * e2 - radius.0) + e2;

        Box::new(EllipsePlot {
            curr: Point(-radius.0 as i32, 0),
            e2: e2 as i32,
            err: err as i32,
            radius: (radius.0 as i32, radius.1 as i32),
            center: Point(center.0 as i32, center.1 as i32),
            point_buffer: vec![],
        })
    }
}

pub struct EllipsePlot {
    center: Point<i32>,
    curr: Point<i32>,
    e2: i32,
    err: i32,
    point_buffer: Vec<Point<i32>>,
    radius: (i32, i32),
}

impl Iterator for EllipsePlot {
    type Item = Point<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let EllipsePlot {
            ref mut curr,
            ref mut e2,
            ref mut err,
            ref mut point_buffer,
            ref center,
            ref radius,
        } = self;

        // Flush point buffer
        if let Some(point) = point_buffer.pop() {
            return Some(Point(center.0 + point.0, center.1 + point.1));
        }

        // Stop iterating
        if curr.0 > 0 {
            return None;
        }

        // 0 & 1 Quadrants
        let next = Point(center.0 - curr.0, center.1 + curr.1);
        // 2 & 3 Quadrants
        point_buffer.push(Point(-curr.0, -curr.1));
        // 4 & 5 Quadrants
        point_buffer.push(Point(curr.0, -curr.1));
        // 6 & 7 Quadrants
        point_buffer.push(Point(curr.0, curr.1));

        *e2 = 2 * *err;

        if *e2 >= (curr.0 * 2 + 1) * radius.1.pow(2) {
            curr.0 += 1;
            *err += (curr.0 * 2 + 1) * radius.1.pow(2);
        }

        if *e2 <= (curr.1 * 2 + 1) * radius.0.pow(2) {
            curr.1 += 1;
            *err += (curr.1 * 2 + 1) * radius.0.pow(2);
        }

        // TODO: Fix ellipse when radius.0 is less or equal to 1.

        Some(next)
    }
}
