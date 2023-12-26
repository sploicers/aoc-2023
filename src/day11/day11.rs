use std::io::{BufReader, Read};
use itertools::*;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Point {
	pub x: u64,
	pub y: u64
}

struct GalaxyMap {
	pub galaxies: Vec<Point>,
	pub x_vacant: Vec<bool>,
	pub y_vacant: Vec<bool>,
	pub expansion_factor: usize
}

impl GalaxyMap {
	pub fn from(reader: &mut BufReader<Box<dyn Read>>) -> Self {
		let mut buffer = String::new();
		reader.read_to_string(&mut buffer).expect("Failed to read input to string");
		
		let n_cols = buffer
			.find('\n')
			.expect("Fatal - failed to find newline in supposedly multi-line input");
		let n_rows = buffer.len() / n_cols;

		let mut galaxies = vec![];
		let mut x_vacant = vec![true; n_cols];
		let mut y_vacant = vec![true; n_rows];
	
		for (y, line) in buffer.lines().enumerate() {
			for (x, char) in line.chars().enumerate() {
				if char == '#' {
					x_vacant[x] = false;
					y_vacant[y] = false;
					galaxies.push(Point { x: x as u64, y: y as u64 })
				}
			}
		}
		Self { galaxies, x_vacant, y_vacant, expansion_factor: 1 }
	}

	pub fn expand(&self, a: &Point, b: &Point) -> Point {
		let x_expansion = (a.x..b.x)
			.filter(|x| self.x_vacant[*x as usize])
			.count() * self.expansion_factor;

		let y_expansion = (a.y..b.y)
			.filter(|y| self.y_vacant[*y as usize])
			.count() * self.expansion_factor;

		Point {
			x: b.x + x_expansion as u64,
			y: b.y + y_expansion as u64
		}
	}

	pub fn set_expansion_factor(&mut self, val: usize) {
		self.expansion_factor = val
	}

	pub fn all_galaxy_pairs(&self) -> Vec<(&Point, &Point)> {
		self.galaxies.iter().tuple_combinations().unique().collect()
	}
}

pub fn part1(reader: &mut BufReader<Box<dyn Read>>) -> u64 {
	let mut map = GalaxyMap::from(reader);
	map.set_expansion_factor(2);

	map
		.all_galaxy_pairs()
		.iter()
		.map(|(p0, p1)| manhattan_dist(p0, &map.expand(p0, p1)))
		.sum()
}

fn manhattan_dist(p0: &Point, p1: &Point) -> u64 {
	p1.x.abs_diff(p0.x) + p1.y.abs_diff(p0.y) - 1
}