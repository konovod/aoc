use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;

type Pair = (Range<i32>, Range<i32>);

pub fn parse(input: String) -> Vec<Pair> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\d+)\-(\d+),(\d+)\-(\d+)").unwrap();
    }
    return input
        .split("\n")
        .map(|x| -> Pair {
            let cap = REGEX.captures(x).unwrap();
            let v: Vec<i32> = (1..=4)
                .map(|i| cap.get(i).unwrap().as_str().parse::<i32>().unwrap())
                .collect();
            ((v[0]..v[1]), (v[2]..v[3]))
        })
        .collect();
}

pub fn included(small: &Range<i32>, big: &Range<i32>) -> bool {
    small.start >= big.start && small.end <= big.end
}

pub fn part1(input: String) -> usize {
    parse(input)
        .iter()
        .filter(|(r1, r2)| included(r1, r2) || included(r2, r1))
        .count()
}

pub fn overlaps(first: &Range<i32>, second: &Range<i32>) -> bool {
    !(first.start > second.end || first.end < second.start)
}

pub fn part2(input: String) -> usize {
    parse(input)
        .iter()
        .filter(|(r1, r2)| overlaps(r1, r2))
        .count()
}

// pub fn part2(input: String) -> i32 {
//     return 0;
// }
