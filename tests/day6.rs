use aoc;

const INPUT: &str = "123";

#[test]
fn test1() {
    assert_eq!(
        aoc::days::day6::part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
        7
    );
    assert_eq!(
        aoc::days::day6::part1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
        5
    );
    assert_eq!(
        aoc::days::day6::part1("nppdvjthqldpwncqszvftbrmjlhg".to_string()),
        6
    );
    assert_eq!(
        aoc::days::day6::part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
        10
    );
    assert_eq!(
        aoc::days::day6::part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
        11
    );
}

// #[test]
// fn test2() {
//     assert_eq!(aoc::days::day_template::part2(INPUT.to_string()), 0);
// }
