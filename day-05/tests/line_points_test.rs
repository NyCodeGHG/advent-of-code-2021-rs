use day_05::Line;

#[test]
fn line_points() {
    let line = Line::from_string("2,5 -> 2,7").unwrap();
    assert_eq!(
        line.get_line_points(false).unwrap(),
        vec![(2, 5), (2, 6), (2, 7)]
    );
    let line = Line::from_string("5,5 -> 5,2").unwrap();
    assert_eq!(
        line.get_line_points(false).unwrap(),
        vec![(5, 5), (5, 4), (5, 3), (5, 2)]
    );
}

#[test]
fn diagonal_lines() {
    let line = Line::from_string("1,1 -> 3,3").unwrap();
    assert_eq!(
        line.get_line_points(true).unwrap(),
        vec![(1, 1), (2, 2), (3, 3)]
    );
    let line = Line::from_string("9,7 -> 7,9").unwrap();
    assert_eq!(
        line.get_line_points(true).unwrap(),
        vec![(9, 7), (8, 8), (7, 9)]
    );
}
