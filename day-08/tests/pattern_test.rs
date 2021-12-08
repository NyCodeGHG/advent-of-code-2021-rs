use day_08::determine_pattern;

#[test]
fn simple_test() {
    let test_string = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
    let pattern = determine_pattern(test_string);
    println!("{:?}", pattern);
    assert_eq!(pattern.len(), 7);
}
