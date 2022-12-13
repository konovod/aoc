use aoc;

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

#[test]
fn test2() {
    assert_eq!(
        aoc::days::day6::part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
        19
    );
    assert_eq!(
        aoc::days::day6::part2("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
        23
    );
    assert_eq!(
        aoc::days::day6::part2("nppdvjthqldpwncqszvftbrmjlhg".to_string()),
        23
    );
    assert_eq!(
        aoc::days::day6::part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
        29
    );
    assert_eq!(
        aoc::days::day6::part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
        26
    );
}
