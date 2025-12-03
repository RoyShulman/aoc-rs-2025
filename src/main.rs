use std::io::{Read, stdin};

mod day1;

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    stdin().lock().read_to_string(&mut buffer).unwrap();
    buffer
}

fn main() {
    let input = read_from_stdin();
    let result = day1::part2(&input).unwrap();
    println!("{}", result);
}
