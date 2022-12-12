use std::collections::HashSet;

pub fn part1_parse(input: String) -> Vec<char> {
    return input
        .split("\n")
        .map(|x| x.trim())
        .map(|x| -> char {
            let mut in_first: HashSet<char> = HashSet::new();
            let half = x.len() / 2;
            let mut iter = x.chars();
            for _ in 0..half {
                in_first.insert(iter.next().unwrap());
            }
            for c in iter {
                if in_first.contains(&c) {
                    return c;
                }
            }
            panic!("same char not found");
        })
        .collect();
}

pub fn part1(input: String) -> i32 {
    return part1_parse(input)
        .iter()
        .map(|x| {
            if x.is_uppercase() {
                (*x as i32 - 'A' as i32) + 27
            } else {
                (*x as i32 - 'a' as i32) + 1
            }
        })
        .sum();
}

pub fn part2(input: String) -> i32 {
    return 0;
}
