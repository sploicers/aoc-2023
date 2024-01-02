use std::{
    io::{BufReader, Read},
    str::FromStr,
};

struct MySpecialIterator {
    lines: Vec<String>,
    pos: usize,
}

struct MySpecialIteratorMirrored {
    items: Vec<String>,
    mirror_pos: usize,
    left: i32,
    right: i32,
}

impl MySpecialIterator {
    pub fn mirrored_around(&self, pos: usize) -> MySpecialIteratorMirrored {
        MySpecialIteratorMirrored {
            items: self.lines.clone(),
            mirror_pos: pos,
            left: (pos - 1) as i32,
            right: pos as i32,
        }
    }
}

impl Iterator for MySpecialIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.lines.len() - 1 {
            None
        } else {
            let next = Some((
                self.lines[self.pos].clone(),
                self.lines[self.pos + 1].clone(),
            ));
            self.pos = self.pos + 1;
            next
        }
    }
}

impl Iterator for MySpecialIteratorMirrored {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.items.len();

        if self.right == n as i32 || self.left == -1 {
            None
        } else {
            let next = Some((
                self.items[self.left as usize].clone(),
                self.items[self.right as usize].clone(),
            ));
            self.left = self.left - 1;
            self.right = self.right.min((n - 1) as i32) + 1;
            next
        }
    }
}

impl FromStr for MySpecialIterator {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            lines: s.lines().map(String::from).collect(),
            pos: 0,
        })
    }
}

pub fn part1(reader: &mut BufReader<Box<dyn Read>>) -> usize {
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Failed to read input");

    let mut total = 0;
    let horizontal_multiplier = 100;

    for section in buffer.split("\n\n") {
        let lines = section.lines().map(String::from).collect::<Vec<_>>();
        let lines_rotated = rotate(lines.clone());

        let mut vertical_iterator = MySpecialIterator {
            lines: lines.clone(),
            pos: 0,
        };
        let mut horizontal_iterator = MySpecialIterator {
            lines: lines_rotated,
            pos: 0,
        };

        total = total
            + if let Some(horizontal) = get_mirror_line_pos(&mut vertical_iterator) {
                horizontal * horizontal_multiplier
            } else if let Some(vertical) = get_mirror_line_pos(&mut horizontal_iterator) {
                vertical + 1
            } else {
                0
            };
    }
    total
}

fn get_mirror_line_pos(iter: &mut MySpecialIterator) -> Option<usize> {
    iter.position(|(left, right)| left == right)
        .and_then(|pos| {
            iter.mirrored_around(pos + 1)
                .all(|(left, right)| left == right)
                .then_some(pos + 1)
        })
}

fn rotate(lines: Vec<String>) -> Vec<String> {
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
