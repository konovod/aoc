use aoc;

const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn test1() {
    assert_eq!(aoc::days::day4::part1(INPUT.to_string()), 2);
}

// #[test]
// fn test2() {
//     assert_eq!(aoc::days::day4::part2(INPUT.to_string()), 0);
// }
