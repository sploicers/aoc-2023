use std::{
    io::{BufReader, Read},
    str::FromStr,
};

struct MySpecialIterator {
    lines: Vec<String>,
    pos: usize,
}

struct MySpecialIteratorMirrored {
    lines: Vec<String>,
    mirror_pos: usize,
    left: usize,
    right: usize,
}

impl MySpecialIterator {
    pub fn mirrored_around(&self, pos: usize) -> MySpecialIteratorMirrored {
        MySpecialIteratorMirrored {
            lines: self.lines.clone(),
            mirror_pos: pos,
            left: pos - 1,
            right: pos,
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
        if self.left == 0 || self.right > self.lines.len() - 1 {
            None
        } else {
            let next = Some((
                self.lines[self.left].clone(),
                self.lines[self.right].clone(),
            ));
            self.left = self.left - 1;
            self.right = self.right + 1;
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

        let mut vertical_iterator = MySpecialIterator {
            lines: lines.clone(),
            pos: 0,
        };
        //let mut horizontal_iterator = MySpecialIterator { lines, pos: 0 };

        total =
            total + horizontal_multiplier * get_mirror_line_pos(&mut vertical_iterator).unwrap_or(0)
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
