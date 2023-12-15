use std::{collections::HashMap, f32::consts::PI};

enum HandType {
	FiveOfKind,
	FourOfKind,
	FullHouse,
	ThreeOfKind,
	TwoPair,
	OnePair,
	HighCard
}

struct Hand {
	hand_type: HandType,
	bid: u32,
}

impl HandType {
	pub fn from(s: &str) -> Self {
		let lookup = unique_char_dict(s);
		let num_unique_chars_in_hand = lookup.len();
		let two_unique_cards = num_unique_chars_in_hand == 2;
		let max_count = lookup.into_values().max().expect("Hand should have a maxinum card count");

		match max_count {
			5 => Self::FiveOfKind,
			4 => Self::FourOfKind,
			3 => if two_unique_cards {
				Self::FullHouse
			} else {
				Self::ThreeOfKind
			},
			2 => match num_unique_chars_in_hand {
				3 => Self::TwoPair,
				4 => Self::OnePair,
				_ => panic!("Hand with two of the same card can only have 3 or 4 unique cards"),
			},
			1 => Self::HighCard,
			_ => panic!("Shouldn't get here - a hand only has five cards")
		}
	}
}

fn unique_char_dict(s: &str) -> HashMap<char, i32> {
	s.chars().fold(HashMap::new(), |mut map, c| {
		*map.entry(c).or_insert(0) += 1;
		map
	})
}

pub fn part1() {
	let card_value_lookup: HashMap<String, usize> = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"]
		.iter()
		.enumerate()
		.map(|(i, card)| (String::from(*card), i + 1))
		.collect();
	


}