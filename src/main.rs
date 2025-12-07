// allow because most of the days aren't ran
#![allow(dead_code)]
use std::io::{Read, stdin};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    stdin().lock().read_to_string(&mut buffer).unwrap();
    buffer
}

fn main() {
    let input = read_from_stdin();
    let result = day7::part2(input.trim()).unwrap();
    println!("{}", result);
}
