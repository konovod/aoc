// mod super::days;

use aoc;

#[test]
fn test_hello() {
    assert_eq!(aoc::days::day0::part1("Hello".to_string()), "Hello");
    assert_eq!(aoc::days::day0::part1("".to_string()), "");
}
