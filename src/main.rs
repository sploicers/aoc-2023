use util::get_input_reader;
mod day1;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod util;

fn main() {
    println!("{}", day8::part1(&mut get_input_reader()));
}
