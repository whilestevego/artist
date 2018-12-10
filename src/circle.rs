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

impl Plotable for Circle {
    fn plot(self) -> Plot {
        let curr = Point(0, self.radius as i32);
        let p = 1 - self.radius as i32;

        Box::new(CirclePlot {
            curr,
            origin: self.origin.into(),
            p,
            point_buffer: Vec::with_capacity(7),
        })
    }
}

// Circle Quadrants
//
//           +Y
//      \  7  |  0  /
//       \    |    /
//        \   |   /
//     6   \  |  /   1
//          \ | /
// -X ----------------- +X
//          / | \
//     5   /  |  \   2
//        /   |   \
//       /    |    \
//      /  4  |  3  \
//            -Y

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
            if self.curr.0 > self.curr.1 {
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
            if self.p < 0 {
                self.p += 2 * self.curr.0 + 3;
                self.curr.0 += 1;
            } else {
                self.p += 2 * self.curr.0 - 2 * self.curr.1 + 5;
                self.curr.0 += 1;
                self.curr.1 -= 1;
            };
        }

        next
    }
}
