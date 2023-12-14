fn strip_prefix_and_parse_nums(prefix: &str, line: Option<&String>) -> Vec<u64> {
    line.and_then(|line| line.strip_prefix(prefix))
        .map(|line| split_on_spaces_and_parse_nums(line.trim()))
        .expect(&format!(
            "Line should have prefix '{}' followed by whitespace-separated numbers",
            prefix
        ))
}

fn split_on_spaces_and_parse_nums(s: &str) -> Vec<u64> {
    s.split_whitespace()
        .map(|number_str| number_str.parse())
        .flatten()
        .collect()
}

pub fn part1(lines: Vec<String>) -> u64 {
    let times = strip_prefix_and_parse_nums("Time:", lines.first());
    let distances = strip_prefix_and_parse_nums("Distance:", lines.last());

    todo!()
}
