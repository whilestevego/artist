extern crate graphics;

use graphics::*;

// Cartesian Quadrants
//         |
//    2    |    1
//         |
// -----------------
//         |
//    3    |    4
//         |

#[test]
fn it_draws_two_lines_at_90_deg() {
    let poly_line = PolyLine::new(vec![(0.0, 0.0), (3.0, 3.0), (0.0, 3.0)]);
    let points = poly_line.plot().collect::<Vec<_>>();

    assert_eq!(
        points,
        vec![
            (0.0, 0.0),
            (1.0, 1.0),
            (2.0, 2.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (1.0, 3.0),
            (0.0, 3.0)
        ]
    )
}

#[test]
fn it_draws_a_square_in_first_quadrant() {
    let poly_line = PolyLine::new(vec![
        (0.0, 0.0),
        (3.0, 0.0),
        (3.0, 3.0),
        (0.0, 3.0),
        (0.0, 0.0),
    ]);

    assert_eq!(
        poly_line.plot().collect::<Vec<_>>(),
        vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(3.0, 1.0),
            Point::new(3.0, 2.0),
            Point::new(3.0, 3.0),
            Point::new(2.0, 3.0),
            Point::new(1.0, 3.0),
            Point::new(0.0, 3.0),
            Point::new(0.0, 2.0),
            Point::new(0.0, 1.0),
            Point::new(0.0, 0.0)
        ]
    )
}

#[test]
fn it_draws_a_diamond_accross_all_quadrants() {
    let poly_line = PolyLine::new(vec![
        (-2.0, 0.0),
        (0.0, 2.0),
        (2.0, 0.0),
        (0.0, -2.0),
        (-2.0, 0.0),
    ]);

    assert_eq!(
        poly_line.plot().collect::<Vec<_>>(),
        vec![
            Point::new(-2.0, 0.0),
            Point::new(-1.0, 1.0),
            Point::new(0.0, 2.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 0.0),
            Point::new(1.0, -1.0),
            Point::new(0.0, -2.0),
            Point::new(-1.0, -1.0),
            Point::new(-2.0, 0.0),
        ]
    )
}
