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

fn build_line_points(a: impl Into<Vector>, b: impl Into<Vector>) -> Vec<Point<i64>> {
    Line::new(a.into(), b.into())
        .plot()
        .take(25)
        .collect::<Vec<_>>()
}

#[test]
fn it_creates_diagonal_line_in_first_quadrant() {
    let points = build_line_points((0.0, 0.0), (3.0, 3.0));
    assert_eq!(vec![(0, 0), (1, 1), (2, 2), (3, 3)], points)
}

#[test]
fn it_creates_diagonal_line_less_than_45_deg() {
    let points = build_line_points((0.0, 0.0), (4.0, 2.0));
    assert_eq!(vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 2)], points)
}

#[test]
fn it_creates_diagonal_line_between_45_and_90_deg() {
    let points = build_line_points((0.0, 0.0), (2.0, 4.0));
    assert_eq!(vec![(0, 0), (1, 1), (1, 2), (2, 3), (2, 4)], points)
}

#[test]

fn it_creates_diagonal_line_in_second_quadrant() {
    let points = build_line_points((0.0, 0.0), (-3.0, 3.0));
    assert_eq!(vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)], points)
}

#[test]

fn it_creates_diagonal_line_in_third_quadrant() {
    let points = build_line_points((0.0, 0.0), (-3.0, -3.0));
    assert_eq!(vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)], points)
}

#[test]

fn it_creates_diagonal_line_in_fourth_quadrant() {
    let points = build_line_points((0.0, 0.0), (3.0, -3.0));
    assert_eq!(vec![(0, 0), (1, -1), (2, -2), (3, -3)], points)
}

#[test]
fn it_creates_line_on_positive_x() {
    let points = build_line_points((0.0, 0.0), (3.0, 0.0));
    assert_eq!(vec![(0, 0), (1, 0), (2, 0), (3, 0)], points)
}

#[test]
fn it_creates_line_on_positive_y() {
    let points = build_line_points((0.0, 0.0), (0.0, 3.0));
    assert_eq!(vec![(0, 0), (0, 1), (0, 2), (0, 3)], points)
}

#[test]
fn it_creates_line_on_negative_x() {
    let points = build_line_points((0.0, 0.0), (-3.0, 0.0));
    assert_eq!(vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)], points)
}

#[test]
fn it_creates_line_on_negative_y() {
    let points = build_line_points((0.0, 0.0), (0.0, -3.0));
    assert_eq!(vec![(0, 0), (0, -1), (0, -2), (0, -3)], points)
}

#[test]

fn it_creates_diagonal_line_from_third_to_first_quadrant() {
    let points = build_line_points((-3.0, -3.0), (3.0, 3.0));
    assert_eq!(
        vec![(-3, -3), (-2, -2), (-1, -1), (0, 0), (1, 1), (2, 2), (3, 3)],
        points
    )
}

#[test]

fn it_creates_diagonal_line_from_first_to_third_quadrant() {
    let points = build_line_points((3.0, 3.0), (-3.0, -3.0));
    assert_eq!(
        vec![(3, 3), (2, 2), (1, 1), (0, 0), (-1, -1), (-2, -2), (-3, -3)],
        points
    )
}

#[test]

fn it_creates_lines_in_arbitrary_angles() {
    let points = build_line_points((1.0, 2.0), (3.0, 10.0));
    assert_eq!(
        vec![
            (1, 2),
            (1, 3),
            (2, 4),
            (2, 5),
            (2, 6),
            (2, 7),
            (3, 8),
            (3, 9),
            (3, 10)
        ],
        points
    )
}

#[test]

fn it_creates_same_points_for_mirrored_lines() {
    let points = build_line_points((-3.0, -3.0), (3.0, 3.0));
    let mut points_mirrored = build_line_points((3.0, 3.0), (-3.0, -3.0));

    points_mirrored.sort();

    assert_eq!(points, points_mirrored)
}

#[test]
fn it_draws_perpendicular_to_x_axis() {
    let points = build_line_points((3.0, 0.0), (3.0, 3.0));
    assert_eq!(vec![(3, 0), (3, 1), (3, 2), (3, 3),], points)
}
