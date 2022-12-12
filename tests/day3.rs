use aoc;

const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn test1_parse() {
    assert_eq!(
        aoc::days::day3::part1_parse(INPUT.to_string()),
        ['p', 'L', 'P', 'v', 't', 's']
    );
}

#[test]
fn test1_value() {
    assert_eq!(aoc::days::day3::part1(INPUT.to_string()), 157);
}

#[test]
fn test2_parse() {
    assert_eq!(aoc::days::day3::part2_parse(INPUT.to_string()), ['r', 'Z']);
}

#[test]
fn test2_value() {
    assert_eq!(aoc::days::day3::part2(INPUT.to_string()), 70);
}
