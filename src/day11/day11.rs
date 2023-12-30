use itertools::*;
use std::io::{BufReader, Read};

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Point {
    pub x: u128,
    pub y: u128,
}

struct GalaxyMap<const N: usize> {
    pub galaxies: Vec<Point>,
    pub x_vacant: Vec<bool>,
    pub y_vacant: Vec<bool>,
}

impl<const N: usize> GalaxyMap<N> {
    pub fn from(reader: &mut BufReader<Box<dyn Read>>) -> Self {
        let mut buffer = String::new();
        reader
            .read_to_string(&mut buffer)
            .expect("Failed to read input to string");

        let n_cols = buffer
            .find('\n')
            .expect("Fatal - failed to find newline in supposedly multi-line input");
        let n_rows = buffer.len() / n_cols;

        let mut galaxies = vec![];
        let mut x_vacant = vec![true; n_cols];
        let mut y_vacant = vec![true; n_rows];

        for (y, line) in buffer.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    x_vacant[x] = false;
                    y_vacant[y] = false;
                    galaxies.push(Point {
                        x: x as u128,
                        y: y as u128,
                    })
                }
            }
        }
        Self {
            galaxies,
            x_vacant,
            y_vacant,
        }
    }

    pub fn expand(&self, a: &Point, b: &Point) -> Point {
        let x_expansion = (a.x..b.x).filter(|x| self.x_vacant[*x as usize]).count() * (N - 1);
        let y_expansion = (a.y..b.y).filter(|y| self.y_vacant[*y as usize]).count() * (N - 1);

        Point {
            x: b.x + x_expansion as u128,
            y: b.y + y_expansion as u128,
        }
    }

    pub fn all_galaxy_pairs(&self) -> Vec<(&Point, &Point)> {
        self.galaxies.iter().tuple_combinations().unique().collect()
    }
}

pub fn part1(reader: &mut BufReader<Box<dyn Read>>) -> u128 {
    let map = GalaxyMap::<2>::from(reader);
    solve(map)
}

pub fn part2(reader: &mut BufReader<Box<dyn Read>>) -> u128 {
    let map = GalaxyMap::<1000000>::from(reader);
    solve(map)
}

fn solve<const N: usize>(map: GalaxyMap<N>) -> u128 {
    map.all_galaxy_pairs()
        .iter()
        .map(|(p0, p1)| manhattan_dist(&map.expand(p1, p0), &map.expand(p0, p1)))
        .sum()
}

fn manhattan_dist(p0: &Point, p1: &Point) -> u128 {
    p1.x.abs_diff(p0.x) + p1.y.abs_diff(p0.y)
}
