use once_cell::sync::Lazy;
use std::{cmp::Ordering, collections::HashMap};

type Cards = [char; 5];
static CARD_VALUES: Lazy<HashMap<char, u32>> =
    Lazy::new(|| HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', 11), ('T', 10)]));

static CARD_VALUES_PART2: Lazy<HashMap<char, u32>> =
    Lazy::new(|| HashMap::from([('A', 13), ('K', 12), ('Q', 11), ('T', 10), ('J', 1)]));

static PART2: bool = true;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: Cards,
    bid: u32,
}

impl HandType {
    pub fn from(s: &str) -> Self {
        if PART2 {
            let labels = CARD_VALUES_PART2
                .clone()
                .into_keys()
                .chain((0..=9).map(|n| char::from_digit(n, 10).unwrap()))
                .collect::<Vec<_>>();

            let potential_hand_types = labels
                .iter()
                .map(|label| {
                    let new_str = s.replace('J', &label.to_string());

                    let lookup = unique_char_dict(&new_str);
                    let num_unique_cards = lookup.len();

                    let max_duplicates = lookup
                        .into_values()
                        .max()
                        .expect("Hand should have a maxinum card count");

                    determine_hand_type(max_duplicates, num_unique_cards)
                })
                .collect::<Vec<_>>();

            *potential_hand_types
                .iter()
                .max()
                .expect("Failed to maximise hand type via joker")
        } else {
            let lookup = unique_char_dict(s);
            let num_unique_cards = lookup.len();

            let max_duplicates = lookup
                .into_values()
                .max()
                .expect("Hand should have a maxinum card count");

            determine_hand_type(max_duplicates, num_unique_cards)
        }
    }
}

impl Hand {
    pub fn from(s: String) -> Self {
        let (hand_type, cards, bid) = s
            .split_once(" ")
            .map(|(content, bid)| {
                let hand_type = HandType::from(content);
                let bid = bid.parse().expect("Bid value should be numeric");
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            if self.hand_type == other.hand_type {
                let hand_size = self.cards.len();

                (0..hand_size)
                    .find(|pos| self.cards[*pos] != other.cards[*pos])
                    .map(|differing_position| {
                        get_card_value(self.cards[differing_position])
                            .cmp(&get_card_value(other.cards[differing_position]))
                    })
                    .unwrap_or(Ordering::Equal)
            } else {
                self.hand_type.cmp(&other.hand_type)
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
            let mut own = self.cards.clone();
            let mut others = other.cards.clone();
            own.sort();
            others.sort();
            own == others
        } else {
            false
        }
    }
}

impl Eq for Hand {}

fn unique_char_dict(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    })
}

fn determine_hand_type(max_duplicates: usize, num_unique_cards: usize) -> HandType {
    match max_duplicates {
        5 => HandType::FiveOfKind,
        4 => HandType::FourOfKind,
        3 => {
            if num_unique_cards == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfKind
            }
        }
        2 => match num_unique_cards {
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            _ => panic!("Hand with two of the same card can only have 3 or 4 unique cards"),
        },
        1 => HandType::HighCard,
        _ => panic!("Shouldn't get here - a hand always contains cards"),
    }
}

fn get_card_value(card: char) -> u32 {
    if let Some(num) = card.to_digit(10) {
        num
    } else {
        let val = if PART2 {
            &CARD_VALUES_PART2
        } else {
            &CARD_VALUES
        }
        .get(&card)
        .expect("Card not found in map");

        *val
    }
}

pub fn part1(lines: impl Iterator<Item = String>) -> u64 {
    let mut hands = lines.map(Hand::from).collect::<Vec<_>>();
    hands.sort();

    let mut total = 0;

    for (i, hand) in hands.iter().enumerate() {
        let rank = (i + 1) as u32;
        println!("{} - {:?}", rank, hand);
        total += (rank * hand.bid) as u64
    }
    total
}

pub fn part2(lines: impl Iterator<Item = String>) -> u64 {
    if !PART2 {
        panic!("Invoked part2 without setting flag");
    }
    part1(lines)
}
