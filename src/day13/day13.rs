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
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Failed to read input");

    buffer.split("\n\n").map(solve_single_mirror).sum()
}

fn solve_single_mirror(section: &str) -> usize {
    let lines = section.lines().collect::<Vec<_>>();
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

    if let Some(horizontal) = get_mirror_line_pos(&mut vertical_iterator) {
        horizontal * HORIZONTAL_MULTIPLIER
    } else if let Some(vertical) = get_mirror_line_pos(&mut horizontal_iterator) {
        rot_len - vertical
    } else {
        0
    }
}

fn get_mirror_line_pos(iter: &mut MySpecialIterator) -> Option<usize> {
    let mut potential_mirror_lines = iter
        .clone()
        .enumerate()
        .filter_map(|(i, (left, right))| (left == right).then_some(i + 1));

    potential_mirror_lines.find(|pos| {
        iter.mirrored_around(*pos)
            .all(|(left, right)| left == right)
    })
}

fn rotate<'a>(lines: &Vec<&'a str>) -> Vec<String> {
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
