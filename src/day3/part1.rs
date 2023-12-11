use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq)]
enum Tile {
    Symbol,
    Part(u32),
}

#[derive(Debug)]
struct Grid {
    coordinates: HashMap<(usize, usize), Tile>,
}

impl Grid {
    pub fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut coordinates: HashMap<(usize, usize), Tile> = HashMap::new();

        for (y, line) in lines.enumerate() {
            let mut x = 0;
            let mut last_num_start_pos: Option<usize> = None;

            loop {
                let current_slice = &mut line[x..].chars();

                if let Some(current_char) = current_slice.next() {
                    if current_char.is_numeric() {
                        if last_num_start_pos.is_none() {
                            last_num_start_pos = Some(x);
                        }
                    } else {
                        if current_char != '.' {
                            coordinates.insert((x, y), Tile::Symbol);
                        }
                        if let Some(num_start) = last_num_start_pos {
                            let part_number: u32 = String::from_iter(line[num_start..x].chars())
                                .parse()
                                .expect("Fatal - failed to parse supposed part number");

                            for num_x in num_start..x {
                                coordinates.insert((num_x, y), Tile::Part(part_number));
                            }
                            last_num_start_pos = None;
                        }
                    }
                    x += 1;
                } else {
                    break;
                }
            }
        }

        Self { coordinates }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.coordinates.get(&(x, y))
    }

    pub fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = &Tile> {
        let mut neighbours: Vec<Option<&Tile>> = vec![];
        for y0 in -1..=1 {
            for x0 in -1..=1 {
                if !(x == 0 && y == 0) {
                    neighbours.push(self.get((x as i32 + x0) as usize, (y as i32 + y0) as usize))
                }
            }
        }
        neighbours.into_iter().flatten()
    }

    pub fn part_number_total(&self) -> u32 {
        let mut total = 0;
        let mut parts_uniq: HashSet<&Tile> = HashSet::new();

        for ((x, y), tile) in self.coordinates.iter() {
            match tile {
                Tile::Symbol => {
                    for neighbour in self.neighbours(*x, *y) {
                        match neighbour {
                            Tile::Part(_) => {
                                parts_uniq.insert(neighbour);
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
        for t in parts_uniq {
            match t {
                Tile::Part(part_num) => total += part_num,
                _ => (),
            }
        }
        total
    }
}

pub fn solution(lines: impl Iterator<Item = String>) -> u32 {
    Grid::from_lines(lines).part_number_total()
}
