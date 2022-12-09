// mod super::days;

use aoc;

const guide: &str = "A Y
B X
C Z";

#[test]
fn test_part1() {
    assert_eq!(aoc::days::day2::part1(guide.to_string()), 15);
}
