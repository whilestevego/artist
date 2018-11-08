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
fn it_creates_diagonal_line_in_first_quadrant() {
    let points = Line::new((0.0, 0.0), (3.0, 3.0)).plot().collect::<Vec<_>>();

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
        .plot()
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
        .plot()
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
        .plot()
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
    let points = Line::new((0.0, 0.0), (3.0, 0.0)).plot().collect::<Vec<_>>();

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
    let points = Line::new((0.0, 0.0), (0.0, 3.0)).plot().collect::<Vec<_>>();

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
        .plot()
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
        .plot()
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
        .plot()
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
        .plot()
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

#[test]
fn it_creates_lines_in_arbitrary_angles() {
    let points = Line::new((1.0, 2.0), (3.0, 10.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            Point::new(1.0, 2.0),
            Point::new(1.25, 3.0),
            Point::new(1.5, 4.0),
            Point::new(1.75, 5.0),
            Point::new(2.0, 6.0),
            Point::new(2.25, 7.0),
            Point::new(2.5, 8.0),
            Point::new(2.75, 9.0),
            Point::new(3.0, 10.0)
        ],
        points
    )
}

#[test]
fn it_creates_same_points_for_mirrored_lines() {
    let points = Line::new((-3.0, -3.0), (3.0, 3.0))
        .plot()
        .collect::<Vec<_>>();

    let mut points_mirrored = Line::new((3.0, 3.0), (-3.0, -3.0))
        .plot()
        .collect::<Vec<_>>();

    points_mirrored.sort();

    assert_eq!(points, points_mirrored)
}
