use line::*;
use point::*;
use std::f64::consts;

#[derive(Debug)]
pub struct Turtle {
    facing: f64,
    corners: Vec<Point>,
}

impl Default for Turtle {
    fn default() -> Self {
        Turtle {
            facing: 0.0,
            corners: vec![Point::new(0.0, 0.0)],
        }
    }
}

impl Turtle {
    pub fn new(begin: impl Into<Point>, facing: f64) -> Self {
        Turtle {
            facing,
            corners: vec![begin.into()],
        }
    }
    pub fn face(&mut self, new_facing: f64) -> &mut Self {
        self.facing = new_facing;
        self
    }
    pub fn turn(&mut self, by: f64) -> &mut Self {
        self.facing += by;
        self
    }
    pub fn forward(&mut self, steps: f64) -> &mut Self {
        let &Point { x, y } = self
            .corners
            .last()
            .expect("Turtle's points must not be empty");

        self.corners.push(Point::new(
            x + steps * (self.facing * consts::PI / 180.0).cos(),
            y + steps * (self.facing * consts::PI / 180.0).sin(),
        ));

        self
    }
    pub fn lines<'a>(&'a self) -> impl Iterator<Item = Line> + 'a {
        self.corners
            .windows(2)
            .map(|line_points| match line_points {
                &[point_a, point_b] => Line::new(point_a, point_b),
                _ => panic!("Unexpected slice size (must be length of 2)"),
            })
    }
    pub fn plot<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        self.lines().flat_map(|line| line.plot())
    }
}
