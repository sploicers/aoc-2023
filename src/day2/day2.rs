use std::error::Error;

pub fn part1(lines: impl Iterator<Item = String>) -> u32 {
    let red = 12;
    let green = 13;
    let blue = 14;

    lines
        .filter_map(|line| {
            let game = Game::from(line);
            game.possible_with(red, green, blue).then_some(game.id)
        })
        .sum()
}

pub fn part2(lines: impl Iterator<Item = String>) -> u32 {
    lines
        .map(|line| {
            let (r, g, b) = Game::from(line).min_possible();
            r * g * b
        })
        .sum()
}

#[derive(Debug)]
pub struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn from(line: String) -> Self {
        let (prefix, rest) = line
            .split_once(": ")
            .expect("Input stream contained malformed line!");

        let id: u32 = prefix.split_once(" ").unwrap().1.parse().unwrap();

        let rounds: Vec<Round> = rest
            .split("; ")
            .map(|s| Round::from_comma_separated_colors(s).unwrap())
            .collect();

        Self { id, rounds }
    }

    pub fn possible_with(&self, red: u32, green: u32, blue: u32) -> bool {
        let (max_r, max_g, max_b) = self.min_possible();
        max_r <= red && max_g <= green && max_b <= blue
    }

    pub fn min_possible(&self) -> (u32, u32, u32) {
        let mut r_max = 0;
        let mut g_max = 0;
        let mut b_max = 0;

        for Round { red, green, blue } in &self.rounds {
            r_max = r_max.max(*red);
            g_max = g_max.max(*green);
            b_max = b_max.max(*blue);
        }
        (r_max, g_max, b_max)
    }
}

impl Round {
    pub fn from_comma_separated_colors(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for entry in s.split(", ") {
            match entry.split_once(" ").unwrap() {
                (n, "red") => red = n.parse()?,
                (n, "green") => green = n.parse()?,
                (n, "blue") => blue = n.parse()?,
                _ => panic!("Got invalid color value in input!"),
            }
        }
        Ok(Self { red, green, blue })
    }
}
