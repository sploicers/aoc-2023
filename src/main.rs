use util::get_input_reader;
mod day1;
mod day3;
mod day4;
mod day5;
mod util;

fn main() {
    println!("{}", day5::part1(&mut get_input_reader()));
}
