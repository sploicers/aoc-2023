pub fn part1(lines: impl Iterator<Item = String>) -> u64 {
    lines
        .map(|line| next(split_on_spaces_and_parse_nums(&line)))
        .sum()
}

fn next(values: Vec<u64>) -> u64 {
    if values.iter().all(|val| *val == 0) {
        0
    } else {
        values
            .last()
            .map(|val| {
                let diffs = values
                    .windows(2)
                    .map(|pair| pair[1].abs_diff(pair[0]))
                    .collect::<Vec<_>>();

                val + next(diffs)
            })
            .expect("Values should be non-empty")
    }
}

fn split_on_spaces_and_parse_nums(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .map(|number_str| number_str.parse())
        .flatten()
        .collect()
}
