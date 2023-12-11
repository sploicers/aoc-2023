use std::{io::{stdin, BufRead}, collections::HashMap};
use fancy_regex::Regex;

pub fn solution() {
	let number_names = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
	let mut number_lookup = HashMap::new();
	let r_str = format!(r"(?=((\d+)|{}))", number_names.map(|name| format!("({})", name)).join("|"));
	println!("{}", r_str);
	let regex = Regex::new(r_str.as_str()).expect("Regex was wrong!");
	let mut sum: u32 = 0;
	
	for (i, num) in number_names.iter().enumerate() {
		number_lookup.insert(*num, (i as u32) + 1);
	}

	for line in stdin().lock().lines() {
		if let Ok(line) = line {
			for r in regex.find_iter(&line) {
				println!("{:?}", r.unwrap());
			}

			let matches: Vec<u32> = regex.find_iter(&line).map(|regex_match| number_name_to_value(regex_match.unwrap().as_str())).collect();

			//println!("{:?}", matches);

			sum += matches
				.first()
				.and_then(|n| matches.last().map(|m| format!("{}{}", n, m)))
				.unwrap_or_default()
				.parse::<u32>()
				.unwrap_or_default();
		}


	}

	print!("{}", sum)
}

fn number_name_to_value(name: &str) -> u32 {
	match name {
		"one" => 1,
		"two" => 2,
		"three" => 3,
		"four" => 4,
		"five" => 5,
		"six" => 6,
		"seven" => 7,
		"eight" => 8,
		"nine" => 9,
		_ => name.parse().unwrap_or(0) 
	}
}