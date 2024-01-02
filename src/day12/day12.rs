use michie::memoized;
use std::collections::HashMap;
use std::str::FromStr;

const OPERATIONAL: char = '.';
const BROKEN: char = '#';
const UNKNOWN: char = '?';

pub fn part1(lines: impl Iterator<Item = String>) -> u64 {
    lines
        .map(|line| num_valid_arrangements(line.parse().expect("Failed to parse input line")))
        .sum()
}
pub fn part2(lines: impl Iterator<Item = String>) -> u64 {
    let folding_factor = 5;

    lines
        .map(|line| {
            num_valid_arrangements(
                unfold_record(line, folding_factor)
                    .parse()
                    .expect("Failed to parse input line"),
            )
        })
        .sum()
}

#[memoized(key_expr = state.clone(), store_type = HashMap<SpringState, u64>)]
fn num_valid_arrangements(state: SpringState) -> u64 {
    let no_runs_left = state.run_lengths.is_empty();
    let no_springs_left = state.springs.is_empty();
    let no_broken_springs_left = !state.springs.contains(&BROKEN);

    if no_springs_left {
        return no_runs_left as u64;
    }
    if no_runs_left {
        return no_broken_springs_left as u64;
    }

    let num_arrangements_if_broken = match state.current_spring() {
        BROKEN | UNKNOWN => {
            if state.can_match_run() {
                let next_state = state.consume_run();
                num_valid_arrangements(next_state)
            } else {
                0
            }
        }
        _ => 0,
    };

    let num_arrangements_if_operational = match state.current_spring() {
        OPERATIONAL | UNKNOWN => {
            // If the spring type is operational here, then this just corresponds to skipping over it and continuing.
            // If the spring type is unknown here, this corresponds to treating it as operational and continuing.
            let next_state = state.consume_single_spring();
            num_valid_arrangements(next_state)
        }
        _ => 0,
    };

    num_arrangements_if_broken + num_arrangements_if_operational
}

fn unfold_record(record: String, folding_factor: usize) -> String {
    record
        .split_once(' ')
        .map(|(spring_section, run_length_section)| {
            format!(
                "{} {}",
                repeat_and_intersperse(spring_section, '?', folding_factor),
                repeat_and_intersperse(run_length_section, ',', folding_factor)
            )
        })
        .expect(&format!("Failed to unfold line '{}'", record))
}

fn repeat_and_intersperse(s: &str, sep: char, n: usize) -> String {
    let original_length = s.len();

    s.repeat(n)
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && i % original_length == 0 {
                Some(sep)
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct SpringState {
    pub springs: Vec<char>,
    pub run_lengths: Vec<usize>,
}

impl SpringState {
    pub fn current_spring(&self) -> char {
        self.springs[0]
    }

    pub fn current_run_length(&self) -> usize {
        self.run_lengths[0]
    }

    pub fn consume_single_spring(&self) -> Self {
        let remaining_springs = self.springs.clone().into_iter().skip(1).collect();
        let remaining_run_lengths = self.run_lengths.clone();

        Self {
            springs: remaining_springs,
            run_lengths: remaining_run_lengths,
        }
    }

    pub fn consume_run(&self) -> Self {
        let run_length = self.current_run_length();
        let next_pos = (run_length + 1).min(self.springs.len() - 1);

        let remaining_springs = self.springs.clone()[next_pos..].to_vec();
        let remaining_run_lengths = self.run_lengths.clone().into_iter().skip(1).collect();

        Self {
            springs: remaining_springs,
            run_lengths: remaining_run_lengths,
        }
    }

    pub fn can_match_run(&self) -> bool {
        let run_length = self.current_run_length();

        // Not enough springs left to satisfy desired run length.
        if self.springs.len() < run_length {
            return false;
        }

        // If there are previous operational springs, then it's not a valid run.
        let mut springs_so_far = self.springs[0..run_length].iter();
        if springs_so_far.any(|spring| *spring == OPERATIONAL) {
            return false;
        }

        // If we've got an operational or unknown spring up next, then we can end the run.
        // If there were a broken spring next, then the run length ought to have been higher.
        self.springs
            .get(run_length)
            .map(|spring| *spring != BROKEN)
            .unwrap_or(run_length >= self.springs.len())
    }
}

impl FromStr for SpringState {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (spring_section, run_length_section) = s
            .split_once(' ')
            .expect("Input lines should be of format '[springs] [counts]");

        let run_lengths: Vec<usize> = run_length_section
            .split(',')
            .flat_map(|c| c.parse())
            .collect();

        let mut springs: Vec<char> = spring_section.chars().collect();
        springs.push(OPERATIONAL);

        Ok(Self {
            springs,
            run_lengths,
        })
    }
}
