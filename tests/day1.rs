// mod super::days;

use aoc;

#[test]
fn test_part1() {
    assert_eq!(
        aoc::days::day1::part1(
            "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000"
                .to_string()
        ),
        24000
    );
}
