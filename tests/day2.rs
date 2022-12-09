// mod super::days;

use aoc;

const GUIDE: &str = "A Y
B X
C Z";

#[test]
fn test1() {
    assert_eq!(aoc::days::day2::part1(GUIDE.to_string()), 15);
}

#[test]
fn test2() {
    assert_eq!(aoc::days::day2::part2(GUIDE.to_string()), 12);
}
