const OPERATIONAL: char = '.';
const BROKEN: char = '#';
const UNKNOWN: char = '?';

pub fn part1(lines: impl Iterator<Item = String>) -> u64 {
    let mut num_ways = 0;
    for line in lines {
        let (spring_section, count_section) = line
            .split_once(' ')
            .expect("Input lines should be of format '[springs] [counts]");

        let run_lengths: Vec<usize> = count_section.split(',').flat_map(|c| c.parse()).collect();
        let mut chars: Vec<char> = spring_section.chars().collect();
        chars.insert(0, OPERATIONAL);
        chars.push(OPERATIONAL);

        let ways = recurse(spring_section.chars().collect(), run_lengths);
        num_ways = num_ways + ways;
    }
    num_ways
}

struct SearchState {
    springs: Vec<char>,
    run_lengths: Vec<usize>,
}

fn recurse(springs: Vec<char>, run_lengths: Vec<usize>) -> u64 {
    let no_runs_left = run_lengths.is_empty();
    let no_springs_left = springs.is_empty();
    let no_broken_springs_left = !springs.contains(&BROKEN);

    if no_springs_left {
        return no_runs_left as u64;
    }
    if no_runs_left {
        return no_broken_springs_left as u64;
    }

    let mut num_ways = 0;
    let spring = springs[0];
    let run_length = run_lengths[0];

    match spring {
        BROKEN | UNKNOWN => {
            if can_match_run(&springs, run_length) {
                if run_length >= springs.len() {
                    num_ways = num_ways + 1;
                } else {
                    let remaining_springs = springs.clone()[(run_length + 1)..].to_vec();
                    let remaining_run_lengths = run_lengths.clone().into_iter().skip(1).collect();
                    num_ways = num_ways + recurse(remaining_springs, remaining_run_lengths);
                }
            }
        }
        OPERATIONAL => {
            let remaining_springs = springs.clone().into_iter().skip(1).collect();
            let remaining_run_lengths = run_lengths.clone();
            num_ways = num_ways + recurse(remaining_springs, remaining_run_lengths);
        }
        _ => panic!("Got unknown spring type '{}' in input.", spring),
    };

    match spring {
        OPERATIONAL | UNKNOWN => {
            let remaining_springs = springs.clone().into_iter().skip(1).collect();
            let remaining_run_lengths = run_lengths.clone();
            num_ways = num_ways + recurse(remaining_springs, remaining_run_lengths);
        }
        _ => (),
    };

    num_ways
}

fn can_match_run(springs: &Vec<char>, run_length: usize) -> bool {
    // Not enough springs left to satisfy desired run length.
    if springs.len() < run_length {
        return false;
    }

    // If there are previous operational springs, then it's not a valid run.
    let mut springs_so_far = springs[0..run_length].iter();
    if springs_so_far.any(|spring| *spring == OPERATIONAL) {
        return false;
    }

    // If we've got an operational or unknown spring up next, then we can end the run.
    // If there were a broken spring next, then the run length ought to have been higher.
    let next_spring_not_broken = springs
        .get(run_length)
        .map(|spring| *spring != BROKEN)
        .unwrap_or(run_length >= springs.len());

    next_spring_not_broken
}
