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
        let ways = recurse(spring_section.chars().collect(), run_lengths);
        num_ways = num_ways + ways;
    }
    num_ways
}

fn recurse(springs: Vec<char>, run_lengths: Vec<usize>) -> u64 {
    let no_runs_left = run_lengths.is_empty();
    let no_broken_springs_left = !springs.contains(&BROKEN);

    if no_runs_left {
        return no_broken_springs_left as u64;
    }
    if springs.is_empty() {
        return no_runs_left as u64;
    }

    let mut num_ways = 0;
    let length = run_lengths[0];

    num_ways = num_ways
        + if springs.starts_with(&[BROKEN]) {
            if accepted(&springs, length) && run_lengths.len() > 0 {
                let remaining_springs = springs.clone()[length + 1..].to_vec();
                let remaining_run_lengths = run_lengths.clone()[1..].to_vec();
                recurse(remaining_springs, remaining_run_lengths)
            } else {
                0
            }
        } else if springs.starts_with(&[OPERATIONAL]) {
            let remaining_springs = springs
                .clone()
                .into_iter()
                .skip_while(|spring| *spring == OPERATIONAL)
                .collect();

            recurse(remaining_springs, run_lengths)
        } else {
            let mut ways = 0;
            for spring_state in [BROKEN, OPERATIONAL] {
                let mut remaining_springs = springs.clone()[length + 1..].to_vec();
                let remaining_run_lengths = run_lengths.clone()[1..].to_vec();
                remaining_springs[0] = spring_state;
                ways = ways + recurse(remaining_springs, remaining_run_lengths)
            }
            ways
        };

    num_ways
}

fn accepted(springs: &Vec<char>, run_length: usize) -> bool {
    // Not enough springs left to satisfy desired run length.
    if springs.len() < run_length {
        return false;
    }

    // If there are operational springs, then it's not a valid run.
    let mut first_run_length_springs = springs[0..run_length].iter();
    if first_run_length_springs.any(|spring| *spring == OPERATIONAL) {
        return false;
    }

    // If we've got an operational or unknown spring up next, then we can end the run.
    // If there were a broken spring next, then the run length ought to have been higher.
    springs
        .get(run_length)
        .map(|spring| *spring != BROKEN)
        .unwrap_or_default()
}
