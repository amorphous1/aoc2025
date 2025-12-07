use std::fs;
use std::time::SystemTime;

mod day01;
mod day02;
mod day03;
mod day04;
mod day06;
mod day07;

fn main() -> Result<(), std::io::Error> {
    let day01_input = &fs::read_to_string("inputs/day01.txt")?;
    print_and_time("day 1 part 1", || day01::part1(day01_input));
    print_and_time("day 1 part 2", || day01::part2(day01_input));

    let day02_input = &fs::read_to_string("inputs/day02.txt")?;
    print_and_time("day 2 part 1", || day02::part1(day02_input));
    print_and_time("day 2 part 2", || day02::part2(day02_input));

    let day03_input = &fs::read_to_string("inputs/day03.txt")?;
    print_and_time("day 3 part 1", || day03::part1(day03_input));
    print_and_time("day 3 part 2", || day03::part2(day03_input));

    let day04_input = &fs::read_to_string("inputs/day04.txt")?;
    print_and_time("day 4 part 1", || day04::part1(day04_input));
    print_and_time("day 4 part 2", || day04::part2(day04_input));

    let day06_input = &fs::read_to_string("inputs/day06.txt")?;
    print_and_time("day 6 part 1", || day06::part1(day06_input));  // 5873191732773
    print_and_time("day 6 part 2", || day06::part2(day06_input));  // 11386445308378

    let day07_input = &fs::read_to_string("inputs/day07.txt")?;
    print_and_time("day 7 part 1", || day07::part1(day07_input));  // 1579
    print_and_time("day 7 part 2", || day07::part2(day07_input));  // 13418215871354

    Ok(())
}

fn print_and_time<F: FnOnce() -> T, T: std::fmt::Debug>(text: &str, f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    println!("{}: {:?} (took {:?})", text, result, start.elapsed().unwrap());
    result
}
