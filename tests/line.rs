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

    assert_eq!(vec![(0, 0), (1, 1), (2, 2), (3, 3)], vectors)
}

#[test]
fn it_creates_diagonal_line_less_than_45_deg() {
    let vectors = Line::new((0.0, 0.0), (4.0, 2.0)).plot().collect::<Vec<_>>();

    assert_eq!(vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 2)], vectors)
}

#[test]
fn it_creates_diagonal_line_between_45_and_90_deg() {
    let vectors = Line::new((0.0, 0.0), (2.0, 4.0)).plot().collect::<Vec<_>>();

    assert_eq!(vec![(0, 0), (1, 1), (1, 2), (2, 3), (2, 4)], vectors)
}

#[test]

fn it_creates_diagonal_line_in_second_quadrant() {
    let vectors = Line::new((0.0, 0.0), (-3.0, 3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)], vectors)
}

#[test]

fn it_creates_diagonal_line_in_third_quadrant() {
    let vectors = Line::new((0.0, 0.0), (-3.0, -3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)], vectors)
}

#[test]

fn it_creates_diagonal_line_in_fourth_quadrant() {
    let vectors = Line::new((0.0, 0.0), (3.0, -3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(vec![(0, 0), (1, -1), (2, -2), (3, -3)], vectors)
}

#[test]
fn it_creates_line_on_positive_x() {
    let vectors = Line::new((0.0, 0.0), (3.0, 0.0))
        .plot()
        .take(4)
        .collect::<Vec<_>>();

    assert_eq!(vec![(0, 0), (1, 0), (2, 0), (3, 0)], vectors)
}

#[test]
fn it_creates_line_on_positive_y() {
    let vectors = Line::new((0.0, 0.0), (0.0, 3.0))
        .plot()
        .take(4)
        .collect::<Vec<_>>();

    assert_eq!(vec![(0, 0), (0, 1), (0, 2), (0, 3)], vectors)
}

#[test]
fn it_creates_line_on_negative_x() {
    let vectors = Line::new((0.0, 0.0), (-3.0, 0.0))
        .plot()
        .take(4)
        .collect::<Vec<_>>();

    assert_eq!(vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)], vectors)
}

#[test]
fn it_creates_line_on_negative_y() {
    let vectors = Line::new((0.0, 0.0), (0.0, -3.0))
        .plot()
        .take(4)
        .collect::<Vec<_>>();

    assert_eq!(vec![(0, 0), (0, -1), (0, -2), (0, -3)], vectors)
}

#[test]

fn it_creates_diagonal_line_from_third_to_first_quadrant() {
    let vectors = Line::new((-3.0, -3.0), (3.0, 3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![(-3, -3), (-2, -2), (-1, -1), (0, 0), (1, 1), (2, 2), (3, 3)],
        vectors
    )
}

#[test]

fn it_creates_diagonal_line_from_first_to_third_quadrant() {
    let vectors = Line::new((3.0, 3.0), (-3.0, -3.0))
        .plot()
        .collect::<Vec<_>>();

    assert_eq!(
        vec![(3, 3), (2, 2), (1, 1), (0, 0), (-1, -1), (-2, -2), (-3, -3)],
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

#[test]
fn it_draws_perpendicular_to_x_axis() {
    let vectors = Line::new((3.0, 0.0), (3.0, 3.0))
        .plot()
        .take(20)
        .collect::<Vec<_>>();

    assert_eq!(vec![(3, 0), (3, 1), (3, 2), (3, 3),], vectors)
}
