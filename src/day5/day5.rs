use std::{
	collections::HashMap,
	io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
struct Range {
	start: u64,
	end: u64,
}

#[derive(Debug)]
struct SeedMap {
	pub source: String,
	pub dest: String,
	ranges: Vec<(Range, Range)>,
}

impl Range {
	pub fn includes(&self, num: u64) -> bool {
		num >= self.start && num < self.end
	}

	pub fn pair_from(line: &str) -> (Self, Self) {
		let parts: Vec<u64> = line
			.split_whitespace()
			.take(3)
			.filter_map(|part| part.parse::<u64>().ok())
			.collect();

		if let [dest_range_start, src_range_start, len] = &parts[..] {
			let dest_range = Self {
				start: *dest_range_start,
				end: dest_range_start + len,
			};
			let src_range = Self {
				start: *src_range_start,
				end: src_range_start + len,
			};
			(src_range, dest_range)
		} else {
			panic!("Malformed line")
		}
	}
}

impl SeedMap {
	pub fn lookup(&self, num: u64) -> u64 {
		if let Some((src, dest)) = self.ranges.iter().find(|(src, _)| src.includes(num)) {
			let offset = num - src.start;
			dest.start + offset
		} else {
			num
		}
	}

	pub fn reverse_lookup(&self, num: u64) -> u64 {
		if let Some((src, dest)) = self.ranges.iter().find(|(_, dest)| dest.includes(num)) {
			let offset = num - dest.start;
			src.start + offset
		} else {
			num
		}
	}

	pub fn from(section: &str) -> Self {
		let lines: Vec<&str> = section.lines().collect();
		let header = lines.first().expect("Section header should not be empty");

		let (source, dest) = header
			.trim()
			.strip_suffix(" map:")
			.and_then(|rest| rest.split_once("-to-"))
			.expect("Section header should be of the form '[source]-to-[destination] map:'");

		let ranges = lines.into_iter().skip(1).map(Range::pair_from).collect();

		Self {
			source: String::from(source),
			dest: String::from(dest),
			ranges,
		}
	}
}

fn split_on_spaces_and_parse_nums(s: &str) -> Vec<u64> {
	s.split_whitespace()
		.map(|number_str| number_str.parse())
		.flatten()
		.collect()
}

fn read_seed_nums(reader: &mut BufReader<Box<dyn Read>>) -> Vec<u64> {
	let mut header = String::new();

	reader
		.read_line(&mut header)
		.expect("Input should start with a header specifying seed numbers");

	let seed_num_section = header
		.strip_prefix("seeds: ")
		.expect("Header should be of the form 'seeds: X Y Z'");

	split_on_spaces_and_parse_nums(&seed_num_section)
}

fn read_seed_ranges(reader: &mut BufReader<Box<dyn Read>>) -> Vec<Range> {
	read_seed_nums(reader)
		.chunks(2)
		.map(|chunk| {
			let start = *chunk.first().expect("Expected start of seed range");
			let len = *chunk.last().expect("Expected end of seed range");
			let end = start + len;
			Range { start, end }
		})
		.collect()
}

fn read_seed_maps(reader: &mut BufReader<Box<dyn Read>>) -> HashMap<String, SeedMap> {
	let mut buffer = String::new();

	reader
		.read_to_string(&mut buffer)
		.expect("Input should have more content after header");

	buffer
		.split("\n\n")
		.map(|section| {
			let map = SeedMap::from(section.trim());
			(map.dest.clone(), map)
		})
		.collect()
}

pub fn part1(reader: &mut BufReader<Box<dyn Read>>) -> u64 {
	let seed_nums = read_seed_nums(reader);
	let seed_maps = read_seed_maps(reader);
	seed_nums
		.iter()
		.map(|num| chained_lookup_part1(&seed_maps, *num))
		.min()
		.expect("Minimum value should exist")
}

pub fn part2(reader: &mut BufReader<Box<dyn Read>>) -> u64 {
	let seed_ranges = read_seed_ranges(reader);
	let seed_maps = read_seed_maps(reader)
		.into_iter()
		.map(|(_, map)| (map.source.clone(), map))
		.collect();
	let mut location_num: u64 = 0;

	loop {
		let corresponding_seed_num = chained_lookup_part2(&seed_maps, location_num);

		if seed_ranges
			.iter()
			.any(|range| range.includes(corresponding_seed_num))
		{
			return location_num;
		} else {
			location_num += 1;
		}
	}
}

fn chained_lookup_part1(seed_maps: &HashMap<String, SeedMap>, seed_num: u64) -> u64 {
	let mut last_lookup_result = seed_num;
	let targets = [
		"soil",
		"fertilizer",
		"water",
		"light",
		"temperature",
		"humidity",
		"location",
	];

	for target in targets {
		last_lookup_result = seed_maps[target].lookup(last_lookup_result);
	}
	last_lookup_result
}

fn chained_lookup_part2(seed_maps: &HashMap<String, SeedMap>, location_num: u64) -> u64 {
	let mut last_lookup_result = location_num;
	let targets = [
		"seed",
		"soil",
		"fertilizer",
		"water",
		"light",
		"temperature",
		"humidity",
	];

	for target in targets.iter().rev() {
		last_lookup_result = seed_maps[*target].reverse_lookup(last_lookup_result);
	}
	last_lookup_result
}
