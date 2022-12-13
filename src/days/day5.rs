use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
type Pile = Vec<char>;
pub struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

impl Instruction {
    pub fn parse(s: &str) -> Self {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let cap = REGEX.captures(s).unwrap();
        let v: Vec<usize> = (1..=3)
            .map(|i| cap.get(i).unwrap().as_str().parse::<usize>().unwrap())
            .collect();
        Self {
            count: v[0],
            from: v[1] - 1,
            to: v[2] - 1,
        }
    }

    pub fn execute(&self, piles: &mut Vec<Pile>) {
        if self.from == self.to {
            return;
        }

        let tuple = if self.from < self.to {
            let (head, tail) = piles.split_at_mut(self.from + 1);
            (&mut head[self.from], &mut tail[self.to - self.from - 1])
        } else {
            let (head, tail) = piles.split_at_mut(self.to + 1);
            (&mut tail[self.from - self.to - 1], &mut head[self.to])
        };

        let (afrom, ato) = tuple;

        for _ in 0..self.count {
            ato.push(afrom.pop().unwrap());
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.count,
            self.from + 1,
            self.to + 1
        )
    }
}

type Task = (Vec<Pile>, Vec<Instruction>);

pub fn parse(input: String) -> Task {
    let mut piles = Vec::new();
    let mut instructions = Vec::new();
    let mut instr_started = false;
    let mut n = 0;
    for s in input.split("\n") {
        if instr_started {
            instructions.push(Instruction::parse(s));
        } else if s.len() < 2 {
            instr_started = true
        } else {
            if n == 0 {
                n = (s.len() + 1) / 4;
                for _ in 0..n {
                    let v: Vec<char> = Vec::new();
                    piles.push(v);
                }
            }
            //now parse input data
            let mut iter = s.chars();
            for i in 0..n {
                let skip = if i == 0 { 1 } else { 3 };
                let ch = iter.nth(skip).unwrap();
                if ch >= 'A' {
                    piles[i].insert(0, ch);
                }
            }
        }
    }
    return (piles, instructions);
}

pub fn part1(input: String) -> String {
    let (mut piles, instr) = parse(input);
    for it in instr {
        it.execute(&mut piles);
    }
    return piles
        .iter()
        .map(|pile| pile.last().unwrap())
        .collect::<String>();
}

pub fn part2(_input: String) -> i32 {
    return 0;
}
