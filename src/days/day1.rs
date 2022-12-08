pub fn parse(input: String) -> Vec<i32> {
    let mut elves = Vec::new();
    let mut elf = 0;
    for s in input.split("\n").map(|x| x.trim()) {
        match s {
            "" => {
                elves.push(elf);
                // println!("{}", elf);
                elf = 0
            }
            _ => {
                let n = s.parse::<i32>().expect(s);
                elf += n
            }
        }
    }
    elves.push(elf);
    return elves;
}

pub fn part1(input: String) -> i32 {
    return *parse(input).iter().max().unwrap();
}

pub fn part2(input: String) -> i32 {
    let mut elves = parse(input);
    elves.sort_by(|a, b| b.cmp(a));
    return elves.iter().take(3).sum::<i32>();
}
