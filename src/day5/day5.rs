use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct SeedMap {
    pub source: String,
    pub dest: String,
    source_ranges: Vec<Range>,
    dest_ranges: Vec<Range>,
}

impl Range {
    pub fn includes(&self, num: u32) -> bool {
        num >= self.start && num <= self.end
    }

    pub fn pair_from(line: &str) -> (Self, Self) {
        let parts: Vec<u32> = line
            .split_whitespace()
            .take(3)
            .filter_map(|part| part.parse::<u32>().ok())
            .collect();

        if let [dest_range_start, src_range_start, len] = &parts[..] {
            let dest_range = Self {
                start: *dest_range_start,
                end: dest_range_start + len,
            };
            let src_range = Self {
                start: *src_range_start,
                end: src_range_start + len,
            };
            (src_range, dest_range)
        } else {
            panic!("Malformed line")
        }
    }
}

impl SeedMap {
    pub fn lookup(&self, num: u32) -> u32 {
        self.source_ranges
            .iter()
            .find(|range| range.includes(num))
            .map(|range| num - range.end + range.start)
            .unwrap_or(num) // unmapped values are unchanged.
    }

    pub fn from(section: &str) -> Self {
        let lines: Vec<&str> = section.lines().collect();
        let header = lines.first().expect("Section header should not be empty");

        let (source, dest) = header
            .trim()
            .strip_suffix(" map:")
            .and_then(|rest| rest.split_once("-to-"))
            .expect("Section header should be of the form '[source]-to-[destination] map:'");

        let mut source_ranges: Vec<Range> = vec![];
        let mut dest_ranges: Vec<Range> = vec![];
        for (dest_range, src_range) in lines.into_iter().skip(1).map(Range::pair_from) {
            source_ranges.push(src_range);
            dest_ranges.push(dest_range);
        }

        Self {
            source: String::from(source),
            dest: String::from(dest),
            source_ranges,
            dest_ranges,
        }
    }
}

fn split_on_spaces_and_parse_nums(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .map(|number_str| number_str.parse())
        .flatten()
        .collect()
}

pub fn part1(reader: &mut Box<dyn Read>) -> u32 {
    let mut buf_reader = BufReader::new(reader);
    let mut header = String::new();

    buf_reader
        .read_line(&mut header)
        .expect("Input should start with a header specifying seed numbers");

    let seed_num_section = header
        .strip_prefix("seeds: ")
        .expect("Header should be of the form 'seeds: X Y Z'");

    let seed_nums = split_on_spaces_and_parse_nums(&seed_num_section);
    let mut remaining_input = String::new();

    buf_reader
        .read_to_string(&mut remaining_input)
        .expect("Input should have more content after header");

    let maps: HashMap<String, SeedMap> = remaining_input
        .split("\n\n")
        .map(|section| {
            let map = SeedMap::from(section.trim());
            (map.source.clone(), map)
        })
        .collect();

    for n in seed_nums {
        println!("{}: {}", n, maps["seed"].lookup(n))
    }
    todo!()
}
