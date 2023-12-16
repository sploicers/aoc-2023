use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

type Node = (String, String, String);

static NODE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").expect("Invalid regex"));

pub fn part1(reader: &mut BufReader<Box<dyn Read>>) -> u64 {
    let directions = read_directions(reader);
    let map = read_nodes(reader);
    let mut current_pos = "AAA";
    let mut num_moves = 0;

    for dir in directions.iter().cycle() {
        if current_pos == "ZZZ" {
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

pub fn part2(reader: &mut BufReader<Box<dyn Read>>) -> u64 {
    let directions = read_directions(reader);
    let map = read_nodes(reader);
    let mut num_moves = 0;

    let mut current_positions: Vec<&String> =
        map.keys().filter(|node| node.ends_with("A")).collect();

    for dir in directions.iter().cycle() {
        if current_positions.iter().all(|pos| pos.ends_with("Z")) {
            break;
        }

        current_positions = current_positions
            .iter()
            .map(|pos| {
                let (left, right) = &map[*pos];
                match dir {
                    'L' => left,
                    'R' => right,
                    _ => panic!("Directions should consist of only L/R"),
                }
            })
            .collect();

        num_moves += 1;
    }

    num_moves
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
    let caps = NODE_REGEX
        .captures(&line)
        .expect("Line should be of the form (ABC) = (DEF, XYZ)");

    let [pos, left, right]: [String; 3] = caps
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
