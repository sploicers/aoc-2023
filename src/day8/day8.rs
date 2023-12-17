use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

type Node = (String, String, String);
type Map = HashMap<String, (String, String)>;

static NODE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").expect("Invalid regex"));

fn solve(
    start_pos: &str,
    map: &Map,
    directions: &Vec<char>,
    stop_cond: impl Fn(&String) -> bool,
) -> u64 {
    let mut current_pos = &String::from(start_pos);
    let mut num_moves = 0;

    for dir in directions.iter().cycle() {
        if stop_cond(current_pos) {
            break;
        }

        let (left, right) = &map[current_pos];
        current_pos = match dir {
            'L' => left,
            'R' => right,
            _ => panic!("Directions should consist of only L/R"),
        };
        num_moves += 1;
    }

    num_moves
}

pub fn part1(reader: &mut BufReader<Box<dyn Read>>) -> u64 {
    let directions = read_directions(reader);
    let map = read_nodes(reader);
    solve("AAA", &map, &directions, |pos| pos == "ZZZ")
}

pub fn part2(reader: &mut BufReader<Box<dyn Read>>) -> u64 {
    let directions = read_directions(reader);
    let map = read_nodes(reader);
    let current_positions = map.keys().filter(|node| node.ends_with("A"));

    current_positions
        .map(|pos| solve(pos, &map, &directions, |pos| pos.ends_with("Z")))
        .reduce(lcm)
        .expect("Should be able to reduce the step count array to its least-common-multiple")
}

fn read_directions(reader: &mut BufReader<Box<dyn Read>>) -> Vec<char> {
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .expect("Input should begin with a line of directions");

    let _ = reader.read_line(&mut buf);
    buf.trim().chars().collect()
}

fn read_nodes(reader: &mut BufReader<Box<dyn Read>>) -> HashMap<String, (String, String)> {
    reader
        .lines()
        .flat_map(Result::ok)
        .map(|line| {
            let (pos, left, right) = read_node(&line);
            (pos, (left, right))
        })
        .collect()
}

fn read_node(line: &String) -> Node {
    let [pos, left, right]: [String; 3] = NODE_REGEX
        .captures(&line)
        .expect("Line should be of the form (ABC) = (DEF, XYZ)")
        .iter()
        .skip(1)
        .take(3)
        .flatten()
        .map(|m| String::from(m.as_str()))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Failed to extract regex capture groups");

    (pos, left, right)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}
