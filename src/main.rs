#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

use util::read_input_lines;
mod day1;
mod day11;
mod day12;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

fn main() {
    println!("{}", day12::part1(read_input_lines()));
}
