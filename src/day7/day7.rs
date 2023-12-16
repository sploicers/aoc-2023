use once_cell::sync::Lazy;
use std::collections::HashMap;

type Cards = [char; 5];
static CARD_VALUES: Lazy<HashMap<char, u32>> =
    Lazy::new(|| HashMap::from([('A', 13), ('K', 12), ('Q', 11), ('J', 10), ('T', 9)]));

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    bid: u32,
    cards: Cards,
}

impl HandType {
    pub fn from(s: &str) -> Self {
        let lookup = unique_char_dict(s);
        let num_unique_cards_in_hand = lookup.len();
        let max_duplicates = lookup
            .into_values()
            .max()
            .expect("Hand should have a maxinum card count");

        match max_duplicates {
            5 => Self::FiveOfKind,
            4 => Self::FourOfKind,
            3 => {
                if num_unique_cards_in_hand == 2 {
                    Self::FullHouse
                } else {
                    Self::ThreeOfKind
                }
            }
            2 => match num_unique_cards_in_hand {
                3 => Self::TwoPair,
                4 => Self::OnePair,
                _ => panic!("Hand with two of the same card can only have 3 or 4 unique cards"),
            },
            1 => Self::HighCard,
            _ => panic!("Shouldn't get here - a hand always contains cards"),
        }
    }
}

impl Hand {
    pub fn from(s: String) -> Self {
        let (hand_type, cards, bid) = s
            .split_once(" ")
            .map(|(content, bid)| {
                let hand_type = HandType::from(content);
                let bid: u32 = bid.parse::<u32>().expect("Bid value should be numeric");
                let cards = content
                    .chars()
                    .collect::<Vec<_>>()
                    .as_slice()
                    .try_into()
                    .expect("Content should consist of 5 chars");

                (hand_type, cards, bid)
            })
            .expect("Input line should be of the form [hand] [bid]");

        Self {
            hand_type,
            bid,
            cards,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq(other) {
            std::cmp::Ordering::Equal
        } else {
            let mut i = 0;
            while i < self.cards.len()
                && get_card_value(self.cards[i]) == get_card_value(other.cards[i])
            {
                i += 1;
            }
            if i == self.cards.len() {
                std::cmp::Ordering::Equal
            } else {
                self.cards[i].cmp(&other.cards[i])
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
            let mut own = self.cards.clone();
            let mut others = self.cards.clone();
            own.sort();
            others.sort();
            own == others
        } else {
            false
        }
    }
}

impl Eq for Hand {}

fn unique_char_dict(s: &str) -> HashMap<char, i32> {
    s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    })
}

fn get_card_value(card: char) -> u32 {
    if let Some(num) = card.to_digit(10) {
        num
    } else {
        *CARD_VALUES.get(&card).expect("Card not found in map")
    }
}

pub fn part1(lines: impl Iterator<Item = String>) -> u32 {
    let mut hands: Vec<Hand> = lines.map(Hand::from).collect();
    hands.sort();
    let mut total = 0;

    for (i, hand) in hands.iter().rev().enumerate() {
        let rank = (i + 1) as u32;
        total += rank * hand.bid;
    }
    total
}
