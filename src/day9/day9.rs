pub fn part1(lines: impl Iterator<Item = String>) -> i64 {
    lines
        .map(|line| next(split_on_spaces_and_parse_nums(&line)))
        .sum()
}

pub fn part2(lines: impl Iterator<Item = String>) -> i64 {
    lines
        .map(|line| {
            let nums_reversed = split_on_spaces_and_parse_nums(&line)
                .into_iter()
                .rev()
                .collect();

            next(nums_reversed)
        })
        .sum()
}

fn next(values: Vec<i64>) -> i64 {
    let n = values.len() as i64;
    values
        .iter()
        .enumerate()
        .map(|(k, val)| {
            // https://amsi.org.au/ESA_Senior_Years/SeniorTopic1/1c/1c_2content_4 (Observation 4)
            let sign: i64 = (-1i64).pow((n - (k as i64) + 1) as u32) as i64;
            sign * val * n_choose_k(n, k as i64)
        })
        .sum()
}

fn n_choose_k(n: i64, k: i64) -> i64 {
    let mut result = 1;
    for i in 1..=k.min(n - k) {
        result = result * (n + 1 - i) / i;
    }
    result
}

fn split_on_spaces_and_parse_nums(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .map(|number_str| number_str.parse())
        .flatten()
        .collect()
}
