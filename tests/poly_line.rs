extern crate turtle;

use turtle::*;

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
fn it_draws_stuff() {
    let poly_line = PolyLine::new(vec![(6.0, 0.0), (3.0, 10.0), (0.0, 0.0)]);
    let points = poly_line.plot().collect::<Vec<_>>();

    assert_eq!(points, vec![(0.0, 0.0)])
}
