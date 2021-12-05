use day_05::Line;

#[test]
fn line_points() {
    let area = Line::from_string("2,5 -> 2,7").unwrap();
    assert_eq!(area.get_line_points().unwrap(), vec![(2, 5), (2, 6), (2, 7)]);
    let area = Line::from_string("5,5 -> 5,2").unwrap();
    assert_eq!(area.get_line_points().unwrap(), vec![(5, 2), (5, 3), (5, 4), (5, 5)]);
}
