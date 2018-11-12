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
    let vectors = Line::new((0.0, 0.0), (3.0, 3.0)).plot().collect::<Vec<_>>();

    assert_eq!(
        vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0)],
        vectors
    )
}

#[test]
fn it_creates_diagonal_line_in_second_quadrant() {
    let vectors = Line::new((0.0, 0.0), (-3.0, 3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![(0.0, 0.0), (-1.0, 1.0), (-2.0, 2.0), (-3.0, 3.0)],
        vectors
    )
}

#[test]
fn it_creates_diagonal_line_in_third_quadrant() {
    let vectors = Line::new((0.0, 0.0), (-3.0, -3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![(0.0, 0.0), (-1.0, -1.0), (-2.0, -2.0), (-3.0, -3.0)],
        vectors
    )
}

#[test]
fn it_creates_diagonal_line_in_fourth_quadrant() {
    let vectors = Line::new((0.0, 0.0), (3.0, -3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![(0.0, 0.0), (1.0, -1.0), (2.0, -2.0), (3.0, -3.0)],
        vectors
    )
}

#[test]
fn it_creates_line_on_positive_x() {
    let vectors = Line::new((0.0, 0.0), (3.0, 0.0)).plot().collect::<Vec<_>>();

    assert_eq!(
        vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0)],
        vectors
    )
}

#[test]
fn it_creates_line_on_positive_y() {
    let vectors = Line::new((0.0, 0.0), (0.0, 3.0)).plot().collect::<Vec<_>>();

    assert_eq!(
        vec![(0.0, 0.0), (0.0, 1.0), (0.0, 2.0), (0.0, 3.0)],
        vectors
    )
}

#[test]
fn it_creates_line_on_negative_x() {
    let vectors = Line::new((0.0, 0.0), (-3.0, 0.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![(0.0, 0.0), (-1.0, 0.0), (-2.0, 0.0), (-3.0, 0.0)],
        vectors
    )
}

#[test]
fn it_creates_line_on_negative_y() {
    let vectors = Line::new((0.0, 0.0), (0.0, -3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![(0.0, 0.0), (0.0, -1.0), (0.0, -2.0), (0.0, -3.0)],
        vectors
    )
}

#[test]
fn it_creates_diagonal_line_from_third_to_first_quadrant() {
    let vectors = Line::new((-3.0, -3.0), (3.0, 3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            (-3.0, -3.0),
            (-2.0, -2.0),
            (-1.0, -1.0),
            (0.0, 0.0),
            (1.0, 1.0),
            (2.0, 2.0),
            (3.0, 3.0)
        ],
        vectors
    )
}

#[test]
fn it_creates_diagonal_line_from_first_to_third_quadrant() {
    let vectors = Line::new((3.0, 3.0), (-3.0, -3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            (3.0, 3.0),
            (2.0, 2.0),
            (1.0, 1.0),
            (0.0, 0.0),
            (-1.0, -1.0),
            (-2.0, -2.0),
            (-3.0, -3.0)
        ],
        vectors
    )
}

#[test]
fn it_creates_lines_in_arbitrary_angles() {
    let vectors = Line::new((1.0, 2.0), (3.0, 10.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            (1.0, 2.0),
            (1.25, 3.0),
            (1.5, 4.0),
            (1.75, 5.0),
            (2.0, 6.0),
            (2.25, 7.0),
            (2.5, 8.0),
            (2.75, 9.0),
            (3.0, 10.0)
        ],
        vectors
    )
}

#[test]
fn it_creates_same_points_for_mirrored_lines() {
    let vectors = Line::new((-3.0, -3.0), (3.0, 3.0))
        .plot()
        .collect::<Vec<_>>();

    let mut points_mirrored = Line::new((3.0, 3.0), (-3.0, -3.0))
        .plot()
        .collect::<Vec<_>>();

    points_mirrored.sort();

    assert_eq!(vectors, points_mirrored)
}
