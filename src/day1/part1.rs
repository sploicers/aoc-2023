use std::io::{stdin, BufRead};

pub fn solution() {
	let mut sum: u32 = 0;
	for line in stdin().lock().lines() {
		let numeric_chars_only: Vec<u32> = line
			.unwrap()
			.chars()
			.filter_map(|c| c.to_digit(10))
			.collect();

		sum += numeric_chars_only
			.first()
			.and_then(|n| numeric_chars_only.last().map(|m| format!("{}{}", n, m)))
			.unwrap_or_default()
			.parse::<u32>()
			.unwrap_or_default();
	}

	print!("{}", sum)
}