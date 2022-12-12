use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_day(index: i32) -> String {
    let path1 = format!("./input/day{}.dat", index);
    let path = Path::new(&path1);
    let mut file = File::open(path).expect("file not exists");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("reading failed");
    return s;
}
pub mod days;

fn main() {
    // println!("{}", days::day0::part1(read_day(0)));
    println!("{}", days::day1::part1(read_day(1)));
    println!("{}", days::day1::part2(read_day(1)));
    println!("{}", days::day2::part1(read_day(2)));
    println!("{}", days::day2::part2(read_day(2)));
    println!("{}", days::day3::part1(read_day(3)));
    println!("{}", days::day3::part2(read_day(3)));
}
