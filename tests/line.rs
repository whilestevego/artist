extern crate turtle;

use turtle::*;

// Cartesian Quadrants
//         |
//    2    |    1
//         |
// -----------------
//         |
//    3    |    4
//         |

#[test]
fn it_creates_diagonal_line_in_first_quadrant() {
    let points = Line::new((0.0, 0.0), (3.0, 3.0))
        .points()
        .map(|p| p.round())
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 2.0),
            Point::new(3.0, 3.0)
        ],
        points
    )
}

#[test]
fn it_creates_diagonal_line_in_second_quadrant() {
    let points = Line::new((0.0, 0.0), (-3.0, 3.0))
        .points()
        .map(|p| p.round())
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(0.0, 0.0),
            Point::new(-1.0, 1.0),
            Point::new(-2.0, 2.0),
            Point::new(-3.0, 3.0)
        ],
        points
    )
}

#[test]
fn it_creates_diagonal_line_in_third_quadrant() {
    let points = Line::new((0.0, 0.0), (-3.0, -3.0))
        .points()
        .map(|p| p.round())
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(0.0, 0.0),
            Point::new(-1.0, -1.0),
            Point::new(-2.0, -2.0),
            Point::new(-3.0, -3.0)
        ],
        points
    )
}

#[test]
fn it_creates_diagonal_line_in_fourth_quadrant() {
    let points = Line::new((0.0, 0.0), (3.0, -3.0))
        .points()
        .map(|p| p.round())
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, -1.0),
            Point::new(2.0, -2.0),
            Point::new(3.0, -3.0)
        ],
        points
    )
}

#[test]
fn it_creates_line_on_positive_x() {
    let points = Line::new((0.0, 0.0), (3.0, 0.0))
        .points()
        .map(|p| p.round())
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(3.0, 0.0)
        ],
        points
    )
}

#[test]
fn it_creates_line_on_positive_y() {
    let points = Line::new((0.0, 0.0), (0.0, 3.0))
        .points()
        .map(|p| p.round())
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, 1.0),
            Point::new(0.0, 2.0),
            Point::new(0.0, 3.0)
        ],
        points
    )
}

#[test]
fn it_creates_line_on_negative_x() {
    let points = Line::new((0.0, 0.0), (-3.0, 0.0))
        .points()
        .map(|p| p.round())
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(0.0, 0.0),
            Point::new(-1.0, 0.0),
            Point::new(-2.0, 0.0),
            Point::new(-3.0, 0.0)
        ],
        points
    )
}

#[test]
fn it_creates_line_on_negative_y() {
    let points = Line::new((0.0, 0.0), (0.0, -3.0))
        .points()
        .map(|p| p.round())
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(0.0, 0.0),
            Point::new(0.0, -1.0),
            Point::new(0.0, -2.0),
            Point::new(0.0, -3.0)
        ],
        points
    )
}

#[test]
fn it_creates_diagonal_line_from_third_to_first_quadrant() {
    let points = Line::new((-3.0, -3.0), (3.0, 3.0))
        .points()
        .map(|p| p.round())
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(-3.0, -3.0),
            Point::new(-2.0, -2.0),
            Point::new(-1.0, -1.0),
            Point::new(0.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 2.0),
            Point::new(3.0, 3.0)
        ],
        points
    )
}

#[test]
fn it_creates_diagonal_line_from_first_to_third_quadrant() {
    let points = Line::new((3.0, 3.0), (-3.0, -3.0))
        .points()
        .map(|p| p.round())
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(3.0, 3.0),
            Point::new(2.0, 2.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 0.0),
            Point::new(-1.0, -1.0),
            Point::new(-2.0, -2.0),
            Point::new(-3.0, -3.0)
        ],
        points
    )
}
