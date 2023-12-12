use std::collections::HashSet;

struct Card {
    card_number: u32,
    numbers_had: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl Card {
    pub fn from(s: String) -> Option<Self> {
        let (card_label, card_contents) = s.split_once(": ")?;
        let card_number: u32 = card_label
            .strip_prefix("Card ")?
            .trim_start()
            .parse()
            .expect("Error parsing card number from input line");

        let (left, right) = card_contents.split_once(" | ")?;
        let numbers_had = split_on_spaces_and_parse_nums(left);
        let winning_numbers = split_on_spaces_and_parse_nums(right);

        Some(Self {
            card_number,
            numbers_had,
            winning_numbers,
        })
    }

    pub fn value(&self) -> u32 {
        let num_matches = self.match_count();
        let multiplier: u32 = 2;

        if num_matches > 0 {
            multiplier.pow(num_matches - 1)
        } else {
            0
        }
    }

    pub fn match_count(&self) -> u32 {
        self.numbers_had
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }
}

fn split_on_spaces_and_parse_nums(s: &str) -> HashSet<u32> {
    s.split(" ")
        .map(|number_str| number_str.parse())
        .flatten()
        .collect()
}

pub fn part1(lines: impl Iterator<Item = String>) -> u32 {
    lines
        .map(|line| Card::from(line).map(|card| card.value()))
        .flatten()
        .sum()
}

pub fn part2(lines: impl Iterator<Item = String>) -> usize {
    let cards: Vec<Card> = lines.map(|line| Card::from(line)).flatten().collect();
    let initial_card_count = cards.len();

    todo!()
}
