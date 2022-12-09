//use std::marker::Tuple;

use core::panic;
use std::default;

pub fn parse(input: String) -> Vec<(char, char)> {
    return input
        .split("\n")
        .map(|x| x.trim().chars().collect())
        .map(|x: Vec<char>| (x[0], x[2]))
        .collect();
}

pub fn part1(input: String) -> i32 {
    let mut score: i32 = 0;
    for (a, x) in parse(input) {
        let v1 = (a as u8) - ('A' as u8) + 1;
        let v2 = (x as u8) - ('X' as u8) + 1;
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
