use once_cell::sync::Lazy;
use regex::Regex;
use std::io::{BufRead, BufReader, Read};

type Node = (String, String, String);

static NODE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").expect("Invalid regex"));

pub fn part1(reader: &mut BufReader<Box<dyn Read>>) -> u64 {
    let mut directions_line = String::new();
    let directions = read_directions(reader, &mut directions_line);
    let mut num_moves = 0;
    let mut next_pos = String::from("AAA");

    for dir in directions {
        let mut next_line = String::new();
        let (pos, left, right) = read_nodes_until(next_pos, reader, &mut next_line);

        if pos == "ZZZ" {
            break;
        }

        next_pos = match dir {
            'L' => left,
            'R' => right,
            _ => panic!("Directions should only consist of L/R"),
        };
        num_moves += 1;
    }

    num_moves
}

fn read_directions<'a>(
    reader: &mut BufReader<Box<dyn Read>>,
    buf: &'a mut String,
) -> impl Iterator<Item = char> + Clone + 'a {
    reader
        .read_line(buf)
        .expect("Input should begin with a line of directions");

    let mut empty_line = String::new();
    let _ = reader.read_line(&mut empty_line);
    buf.chars().cycle()
}

fn read_nodes_until(goal: String, reader: &mut BufReader<Box<dyn Read>>, buf: &mut String) -> Node {
    loop {
        let (pos, left, right) = read_node(reader, buf);
        if pos == goal {
            return (pos, left, right);
        }
    }
}

fn read_node(reader: &mut BufReader<Box<dyn Read>>, buf: &mut String) -> Node {
    reader.read_line(buf).expect("Input should be non-empty");
    let caps = NODE_REGEX
        .captures(&buf)
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

    buf.clear();
    (pos, left, right)
}
