use std::io::{BufReader, Read};

const HORIZONTAL_MULTIPLIER: usize = 100;

#[derive(Clone)]
struct MySpecialIterator<'a> {
    lines: &'a Vec<&'a str>,
    pos: usize,
}

struct MySpecialIteratorMirrored<'a> {
    items: &'a Vec<&'a str>,
    mirror_pos: usize,
    left: i32,
    right: i32,
}

impl<'a> MySpecialIterator<'a> {
    pub fn mirrored_around(&self, pos: usize) -> MySpecialIteratorMirrored {
        MySpecialIteratorMirrored {
            items: self.lines,
            mirror_pos: pos,
            left: (pos - 1) as i32,
            right: pos as i32,
        }
    }
}

impl<'a> Iterator for MySpecialIterator<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.lines.len() - 1 {
            None
        } else {
            let next = Some((self.lines[self.pos], self.lines[self.pos + 1]));
            self.pos = self.pos + 1;
            next
        }
    }
}

impl<'a> Iterator for MySpecialIteratorMirrored<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.items.len();

        if self.right > (n - 1) as i32 || self.left < 0 {
            None
        } else {
            let next = Some((
                self.items[self.left as usize],
                self.items[self.right as usize],
            ));
            self.left = self.left - 1;
            self.right = self.right + 1;
            next
        }
    }
}

pub fn part1(reader: &mut BufReader<Box<dyn Read>>) -> usize {
    solve(reader, &|left, right| left == right)
}

pub fn part2(reader: &mut BufReader<Box<dyn Read>>) -> usize {
    solve(reader, &differ_by_at_most_one_char)
}

pub fn solve(
    reader: &mut BufReader<Box<dyn Read>>,
    predicate: &impl Fn(&str, &str) -> bool,
) -> usize {
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Failed to read input");

    buffer
        .split("\n\n")
        .map(|mirror| solve_single_mirror(mirror, predicate))
        .sum()
}

fn solve_single_mirror(mirror: &str, predicate: &impl Fn(&str, &str) -> bool) -> usize {
    let lines = mirror.lines().collect::<Vec<_>>();
    let lines_rotated = rotate(&lines);
    let rot_len = lines_rotated.len();

    let mut vertical_iterator = MySpecialIterator {
        lines: &lines,
        pos: 0,
    };
    let mut horizontal_iterator = MySpecialIterator {
        lines: &lines_rotated.iter().map(|s| &**s).collect(),
        pos: 0,
    };

    if let Some(horizontal) = find_mirror_line(&mut vertical_iterator, predicate) {
        horizontal * HORIZONTAL_MULTIPLIER
    } else if let Some(vertical) = find_mirror_line(&mut horizontal_iterator, predicate) {
        rot_len - vertical
    } else {
        0
    }
}

fn find_mirror_line(
    iter: &mut MySpecialIterator,
    predicate: impl Fn(&str, &str) -> bool,
) -> Option<usize> {
    let mut potential_mirror_lines = iter
        .clone()
        .enumerate()
        .filter_map(|(i, (left, right))| predicate(left, right).then_some(i + 1));

    potential_mirror_lines.find(|pos| {
        iter.mirrored_around(*pos)
            .all(|(left, right)| predicate(left, right))
    })
}

fn rotate(lines: &Vec<&str>) -> Vec<String> {
    let width = lines.first().map(|line| line.len()).unwrap_or(0);
    let height = lines.len();
    let lines_joined = lines.join("");

    let chars = lines_joined.chars();
    let mut temp: Vec<char> = vec![];

    for j in 0..width {
        for char in chars.clone().skip(width - j - 1).step_by(width) {
            temp.push(char);
        }
    }

    temp.chunks(height)
        .map(|chars| chars.iter().collect())
        .collect::<Vec<_>>()
}

fn differ_by_at_most_one_char(a: &str, b: &str) -> bool {
    hamming_distance(to_bits(a), to_bits(b)) <= 1
}

fn to_bits(line: &str) -> i32 {
    line.chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("Encountered invalid char '{}' in input", c),
        })
        .fold(0, |acc, n| (acc << 1) | n)
}

fn hamming_distance(x: i32, y: i32) -> u32 {
    (x ^ y).count_ones()
}
