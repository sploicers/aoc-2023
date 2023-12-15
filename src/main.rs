use util::read_input_lines;
mod day1;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod util;

fn main() {
    println!("{}", day6::part1(read_input_lines().collect()));
}
