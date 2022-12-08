pub fn part1(input: String) -> i32 {
    let mut elves = Vec::new();
    let mut elf = 0;
    for s in input.split("\n").map(|x| x.trim()) {
        match s {
            "" => {
                elves.push(elf);
                elf = 0
            }
            _ => {
                let n = s.parse::<i32>().expect(s);
                elf += n
            }
        }
    }
    return *elves.iter().max().unwrap();
}
