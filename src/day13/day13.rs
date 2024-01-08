use std::io::{BufReader, Read};

const HORIZONTAL_MULTIPLIER: usize = 100;

#[derive(Clone)]
pub struct MySpecialIterator<'a> {
	lines: &'a Vec<&'a str>,
	pos: usize,
}

pub struct MySpecialIteratorMirrored<'a> {
	items: &'a Vec<&'a str>,
	mirror_pos: usize,
	left: Option<usize>,
	right: Option<usize>,
}


impl MySpecialIterator<'_> {
	pub fn mirrored_around(&self, pos: usize) -> MySpecialIteratorMirrored {
		MySpecialIteratorMirrored {
			items: self.lines,
			mirror_pos: pos,
			left: pos.checked_sub(1),
			right: Some(pos),
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
		let potential_left = self.left.and_then(|pos| self.items.get(pos));
		let potential_right = self.right.and_then(|pos| self.items.get(pos));

		match (potential_left, potential_right) {
			(Some(left), Some(right)) => {
				let next = Some((*left, *right));
				self.left = self.left.and_then(|left| left.checked_sub(1));
				self.right = self.right.map(|right| right.min(self.items.len()) + 1);
				next
			},
			_ => None
		}
	}
}

impl MySpecialIteratorMirrored<'_> {
	pub fn all_elements_differ_by(&mut self, dist: u32) -> bool {
		self.num_elems_differing_by(dist) == self.items.len()
	}

	pub fn num_elems_differing_by(&mut self, dist: u32) -> usize {
		self.filter(|(a, b)| hamming_distance(a, b) == dist).count()
	}

	pub fn num_elems_differing_by_at_most(&mut self, dist: u32) -> usize {
		self.filter(|(a, b)| hamming_distance(a, b) <= dist).count()
	}

	pub fn current_pos_differs_by(&self, dist: u32) -> bool {
		match (self.left, self.right) {
			(Some(left), Some(right)) => {
				hamming_distance(self.items[left], self.items[right]) == dist
			}
			_ => false
		}
	}
}

pub fn part1(reader: &mut BufReader<Box<dyn Read>>) -> usize {
	solve(reader, &|left, right| hamming_distance(left, right) == 0)
}

pub fn part2(reader: &mut BufReader<Box<dyn Read>>) -> usize {
	solve(reader, &|left, right| hamming_distance(left, right) == 1)
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
	predicate: &impl Fn(&str, &str) -> bool,
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

fn to_bits(line: &str) -> i32 {
	line.chars()
		.map(|c| match c {
			'.' => 0,
			'#' => 1,
			_ => panic!("Encountered invalid char '{}' in input", c),
		})
		.fold(0, |acc, n| (acc << 1) | n)
}

fn hamming_distance(a: &str, b: &str) -> u32 {
	(to_bits(a) ^ to_bits(b)).count_ones()
}
