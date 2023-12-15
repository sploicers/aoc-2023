fn strip_prefix_and_parse_nums(prefix: &str, line: Option<&String>) -> Vec<u64> {
	line.and_then(|line| line.strip_prefix(prefix))
		.map(|line| split_on_spaces_and_parse_nums(line.trim()))
		.expect(&format!(
			"Line should have prefix '{}' followed by whitespace-separated numbers",
			prefix
		))
}

fn split_on_spaces_and_parse_nums(s: &str) -> Vec<u64> {
	s.split_whitespace()
		.map(|number_str| number_str.parse())
		.flatten()
		.collect()
}

fn strip_prefix_join_on_whitespace_and_parse_num(prefix: &str, s: Option<&String>) -> u64 {
	s.and_then(|s| s.strip_prefix(prefix))
		.and_then(|s| s.split_whitespace().collect::<Vec<_>>().join("").parse().ok())
		.expect("Joined line should be numeric")
}

fn lower_bound(distance: u64, time: u64) -> f64 {
	((time as f64) - f64::sqrt((time * time) as f64 - 4. * (distance as f64))) / 2.
}

fn upper_bound(distance: u64, time: u64) -> f64 {
	((time as f64) + f64::sqrt((time * time) as f64 - 4. * (distance as f64))) / 2.
}

fn clamp(min: f64, max: f64) -> (u64, u64) {
	if max.fract() == 0. {
		(min.ceil() as u64 + 1, max as u64 - 1)
	} else {
		(min.ceil() as u64, max as u64)
	}
}

pub fn part1(lines: Vec<String>) -> u64 {
	let times = strip_prefix_and_parse_nums("Time:", lines.first());
	let distances = strip_prefix_and_parse_nums("Distance:", lines.last());

	let races = times.iter().zip(distances);
	let mut product = 1;

	for (duration, distance_record) in races {
		let lower_root = lower_bound(distance_record, *duration);
		let upper_root = upper_bound(distance_record, *duration);
		let (lower, upper) = clamp(lower_root, upper_root);
		let num_ways = upper - lower + 1;
		product *= num_ways;
	}
	product
}

pub fn part2(lines: Vec<String>) -> u64 {
	let time = strip_prefix_join_on_whitespace_and_parse_num("Time:", lines.first());
	let distance = strip_prefix_join_on_whitespace_and_parse_num("Distance:", lines.last());
	let lower_root = lower_bound(distance, time);
	let upper_root = upper_bound(distance, time);
	let (lower, upper) = clamp(lower_root, upper_root);
	let num_ways = upper - lower + 1;
	num_ways
}
