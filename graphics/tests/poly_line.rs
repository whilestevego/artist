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
    let vectors = poly_line.plot().take(25).collect::<Vec<_>>();

    assert_eq!(
        vectors,
        vec![(0, 0), (1, 1), (2, 2), (3, 3), (2, 3), (1, 3), (0, 3)]
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
        vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (3, 1),
            (3, 2),
            (3, 3),
            (2, 3),
            (1, 3),
            (0, 3),
            (0, 2),
            (0, 1),
            (0, 0)
        ],
        poly_line.plot().take(25).collect::<Vec<_>>()
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
        vec![
            (-2, 0),
            (-1, 1),
            (0, 2),
            (1, 1),
            (2, 0),
            (1, -1),
            (0, -2),
            (-1, -1),
            (-2, 0),
        ],
        poly_line.plot().take(25).collect::<Vec<_>>(),
    )
}
