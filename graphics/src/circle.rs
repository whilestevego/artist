use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Circle {
    origin: Vector,
    radius: f32,
}

impl Circle {
    pub fn new(origin: impl Into<Vector>, radius: f32) -> Self {
        Self {
            origin: origin.into(),
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
        let curr = Point(self.radius as i32, 0);
        let p = 1 - self.radius as i32;

        Box::new(CirclePlot {
            curr,
            origin: self.origin.into(),
            p,
            point_buffer: Vec::with_capacity(7),
        })
    }
}

pub struct CirclePlot {
    curr: Point<i32>,
    origin: Point<i32>,
    p: i32,
    point_buffer: Vec<Point<i32>>,
}

impl Iterator for CirclePlot {
    type Item = Point<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let next;

        if let Some(next_point) = self.point_buffer.pop() {
            next = Some(Point(
                next_point.0 + self.origin.0,
                next_point.1 + self.origin.1,
            ))
        } else {
            if self.curr.1 > self.curr.0 {
                return None;
            };

            next = Some(Point(
                self.curr.0 + self.origin.0,
                self.curr.1 + self.origin.1,
            ));

            // (x, y)
            // self.curr
            // (x, -y)
            self.point_buffer.push(Point(self.curr.0, -self.curr.1));
            // (y, -x)
            self.point_buffer.push(Point(self.curr.1, -self.curr.0));
            // (-y, -x)
            self.point_buffer.push(Point(-self.curr.1, -self.curr.0));
            // (-x, -y)
            self.point_buffer.push(Point(-self.curr.0, -self.curr.1));
            // (-x, y)
            self.point_buffer.push(Point(-self.curr.0, self.curr.1));
            // (-y, x)
            self.point_buffer.push(Point(-self.curr.1, self.curr.0));
            // (y, x)
            self.point_buffer.push(Point(self.curr.1, self.curr.0));

            // Calculate next point
            // TODO: pre-calculate multiplications
            if self.p < 0 {
                self.p += 2 * self.curr.1 + 3;
                self.curr.1 += 1;
            } else {
                self.p += 2 * self.curr.1 - 2 * self.curr.0 + 5;
                self.curr.1 += 1;
                self.curr.0 -= 1;
            };
        }

        next
    }
}
