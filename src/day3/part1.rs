use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Tile {
	Symbol, Part(MachinePart)
}

#[derive(Debug)]
struct MachinePart {
	part_num: u32,
}

#[derive(Debug)]
struct Grid {
	coordinates: HashMap<(usize, usize), Tile>
}

impl Grid {
	pub fn from_lines(lines: impl Iterator<Item = String>) -> Self {
		let mut coordinates: HashMap<(usize, usize), Tile> = HashMap::new();

		for (y, line) in lines.enumerate() {
			let mut x = 0;
			let mut last_num_start_pos: Option<usize> = None;

			loop {
				let current_slice = &mut line[x..].chars();

				if let Some(current_char) = current_slice.next() {
					if current_char.is_numeric() {
						if last_num_start_pos.is_none() {
							last_num_start_pos = Some(x);
						}
					} else {
						if current_char == '.' {
							coordinates.insert((x, y), Tile::Symbol);
						}
						if let Some(num_start) = last_num_start_pos {
							let part_number: u32 = String::from_iter(line[num_start..x].chars())
								.parse()
								.expect("Fatal - failed to parse supposed part number");

							for num_x in num_start..=x {
								coordinates.insert((num_x, y), Tile::Part(MachinePart { part_num: part_number }));
							}
							last_num_start_pos = None;
						}
					}
					x += 1;
				} else {
					break;
				}
			}
		}

		Self { coordinates }
	}

	pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
		self.coordinates.get(&(x, y))
	}

	pub fn neighbours(&self, x: usize, y: usize) -> Vec<Tile> {
		todo!()
	}
}


pub fn solution(lines: impl Iterator<Item = String>) -> u32 {
	let grid = Grid::from_lines(lines);
	println!("{:?}", grid);
	0
}