use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Ellipse {
    origin: Vector,
    radii: (f32, f32),
}

impl Ellipse {
    pub fn new(origin: impl Into<Vector>, radii: (f32, f32)) -> Self {
        Self {
            origin: origin.into(),
            radii,
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

/// Implements Midpoint Ellipse Algorith
impl Plotable for Ellipse {
    fn plot(self) -> Plot {
        let Ellipse {
            radii: (rx, ry), ..
        } = self;
        let curr = Point(0, ry as i32);
        let rx_sq = rx.powf(2.0);
        let ry_sq = ry.powf(2.0);
        let two_rx_sq = 2.0 * rx_sq;
        let two_ry_sq = 2.0 * ry_sq;
        let py = two_rx_sq * ry;

        // Decision parameter for quadrant 0
        let p = ry_sq - rx_sq * ry + 0.25 * rx_sq;

        Box::new(EllipsePlot {
            curr,
            origin: self.origin.into(),
            p: p.round() as i32,
            px: 0,
            py: py.round() as i32,
            point_buffer: vec![],
            quadrant: 0,
            rx_sq: rx_sq.round() as i32,
            ry_sq: ry_sq.round() as i32,
            two_rx_sq: two_rx_sq.round() as i32,
            two_ry_sq: two_ry_sq.round() as i32,
        })
    }
}

pub struct EllipsePlot {
    curr: Point<i32>,
    origin: Point<i32>,
    p: i32,
    px: i32,
    py: i32,
    point_buffer: Vec<Point<i32>>,
    quadrant: i32,
    rx_sq: i32,
    ry_sq: i32,
    two_rx_sq: i32,
    two_ry_sq: i32,
}

impl Iterator for EllipsePlot {
    type Item = Point<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let EllipsePlot {
            ref mut curr,
            ref mut p,
            ref mut px,
            ref mut py,
            ref mut point_buffer,
            ref mut quadrant,
            ref origin,
            ref rx_sq,
            ref ry_sq,
            ref two_rx_sq,
            ref two_ry_sq,
        } = self;

        // Output points in buffer
        if let Some(next_point) = point_buffer.pop() {
            return Some(Point(next_point.0 + origin.0, next_point.1 + origin.1));
        }

        if curr.1 < 0 {
            return None;
        }

        // Store mirrored points of all other quadrants in buffer
        // Quadrant 2 & 3
        point_buffer.push(Point(curr.0, -curr.1));
        // Quadrant 4 & 5
        point_buffer.push(Point(-curr.0, -curr.1));
        // Quadrant 6 & 7
        point_buffer.push(Point(-curr.0, curr.1));

        // Set next point
        let next = Some(Point(curr.0 + origin.0, curr.1 + origin.1));

        match quadrant {
            0 => {
                if px < py {
                    // Quadrant 0
                    curr.0 += 1;
                    *px += two_ry_sq;

                    if *p < 0 {
                        *p += ry_sq + *px;
                    } else {
                        *p += ry_sq + *px - *py;

                        curr.1 -= 1;
                        *py -= two_rx_sq;
                    };
                } else {
                    *p = (*ry_sq as f32 * (curr.0 as f32 + 2.0).powf(2.0)
                        + *rx_sq as f32 * (curr.1 as f32 - 1.0).powf(2.0)
                        - *rx_sq as f32 * *ry_sq as f32)
                        .round() as i32;
                    *quadrant = 1;
                }
            }
            // TODO: Begin plotting from (0, ry) instead
            1 => {
                curr.1 -= 1;
                *py -= two_rx_sq;

                if *p > 0 {
                    *p += rx_sq - *py;
                } else {
                    *p += rx_sq + *px - *py;

                    curr.0 += 1;
                    *px += two_ry_sq;
                };
            }
            _ => panic!("Only quadrants 0 & 1 should be reachabled"),
        }

        next
    }
}
