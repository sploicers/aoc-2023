#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

use util::get_input_reader;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day19;
mod util;

fn main() {
    println!("{}", day19::part1(&mut get_input_reader()));
}
