use core::panic;

pub fn parse(input: String) -> Vec<(u8, u8)> {
    return input
        .split("\n")
        .map(|x| x.trim().chars().collect())
        .map(|x: Vec<char>| {
            (
                (x[0] as u8) - ('A' as u8) + 1,
                (x[2] as u8) - ('X' as u8) + 1,
            )
        })
        .collect();
}

pub fn part1(input: String) -> i32 {
    let mut score: i32 = 0;
    for (v1, v2) in parse(input) {
        score += v2 as i32;
        score += match (3 + v2 - v1) % 3 {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => panic!("bug"),
        };
    }
    return score;
}

pub fn part2(input: String) -> i32 {
    let mut score: i32 = 0;
    for (v1, res) in parse(input) {
        let v2 = (3 + v1 + res) % 3;
        score += v2 as i32 + 1;
        score += match res {
            1 => 0,
            2 => 3,
            3 => 6,
            _ => panic!("bug"),
        };
    }
    return score;
}
