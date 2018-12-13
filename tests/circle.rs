use graphics::*;

// Cartesian Quadrants
//         |
//    2    |    1
//         |
// -----------------
//         |
//    3    |    4
//         |

fn generate_circle_points(origin: impl Into<Vector>, radius: f32) -> Vec<Point<i32>> {
    let mut points = Circle::new(origin, radius)
        .plot()
        .take(50)
        .collect::<Vec<_>>();

    // plotted circle points do not follow a useful order
    points.sort();
    points
}

#[test]
fn it_draws_a_circle_centered_origin() {
    let points = generate_circle_points((0.0, 0.0), 2.0);
    assert_eq!(
        vec![
            (-2, -1),
            (-2, 0),
            (-2, 0),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (0, -2),
            (0, -2),
            (0, 2),
            (0, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 0),
            (2, 0),
            (2, 1)
        ],
        points
    )
}

#[test]
fn it_draws_a_circle_in_the_first_quadrant() {
    let points = generate_circle_points((2.0, 2.0), 2.0);
    assert_eq!(
        vec![
            (0, 1),
            (0, 2),
            (0, 2),
            (0, 3),
            (1, 0),
            (1, 4),
            (2, 0),
            (2, 0),
            (2, 4),
            (2, 4),
            (3, 0),
            (3, 4),
            (4, 1),
            (4, 2),
            (4, 2),
            (4, 3)
        ],
        points
    )
}
