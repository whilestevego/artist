use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Circle {
    center: Vector,
    radius: f32,
}

impl Circle {
    pub fn new(center: impl Into<Vector>, radius: f32) -> Self {
        Self {
            center: center.into(),
            radius,
        }
    }
}

// Circle Quadrants
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

/// Implements Midpoint Circle Algorithm
impl Plotable for Circle {
    fn plot(self) -> Plot {
        let Circle { center, radius } = self;
        let err = 2.0 - 2.0 * radius;

        Box::new(CirclePlot {
            center: Point(center.0 as i32, center.1 as i32),
            curr: Point(-radius as i32, 0),
            err: err as i32,
            point_buffer: vec![],
        })
    }
}

pub struct CirclePlot {
    center: Point<i32>,
    curr: Point<i32>,
    err: i32,
    point_buffer: Vec<Point<i32>>,
}

impl Iterator for CirclePlot {
    type Item = Point<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let CirclePlot {
            ref mut curr,
            ref mut err,
            ref mut point_buffer,
            ref center,
        } = self;

        // Flush point buffer
        if let Some(point) = point_buffer.pop() {
            return Some(Point(center.0 + point.0, center.1 + point.1));
        }

        // Stop iterating
        if curr.0 > 0 {
            return None;
        }

        // 0 & 1 Quadrant
        let next = Point(center.0 - curr.0, center.1 + curr.1);
        // 2 & 3 Quadrant
        point_buffer.push(Point(-curr.0, -curr.1));
        // 4 & 5 Quadrant
        point_buffer.push(Point(curr.0, -curr.1));
        // 6 & 7 Quadrant
        point_buffer.push(Point(curr.0, curr.1));

        // Calculate next point and update err
        let r = *err;

        if r <= curr.1 {
            curr.1 += 1;
            *err += curr.1 * 2 + 1;
        }

        if r > curr.0 || *err > curr.1 {
            curr.0 += 1;
            *err += curr.0 * 2 + 1;
        }

        Some(next)
    }
}
