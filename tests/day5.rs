use aoc;

const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn test1() {
    assert_eq!(aoc::days::day5::part1(INPUT.to_string()), "CMZ");
}

// #[test]
// fn test2() {
//     assert_eq!(aoc::days::day_template::part2(INPUT.to_string()), 0);
// }
