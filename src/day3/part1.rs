use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq)]
enum Tile {
    Symbol(char),
    Part(MachinePart),
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct MachinePart {
    pub part_number: u32,
    pub start: usize,
    pub end: usize,
    pub row: usize,
}

impl MachinePart {
    pub fn bounding_box(&self) -> impl Iterator<Item = (i32, i32)> {
        let mut surrounding_points: Vec<(i32, i32)> = vec![];
        let left = self.start as i32 - 1;
        let right = (self.end + 1) as i32;

        // to the left and to the right
        surrounding_points.push((left, self.row as i32));
        surrounding_points.push((right as i32, self.row as i32));

        // above and below
        for x in left..=right {
            surrounding_points.push((x, self.row as i32 - 1));
            surrounding_points.push((x, (self.row + 1) as i32));
        }
        surrounding_points.into_iter()
    }
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
                            coordinates.insert((x, y), Tile::Symbol(current_char));
                        }
                        if let Some(num_start) = last_num_start_pos {
                            let part_number: u32 = String::from_iter(line[num_start..x].chars())
                                .parse()
                                .expect("Fatal - failed to parse supposed part number");

                            for num_x in num_start..x {
                                coordinates.insert(
                                    (num_x, y),
                                    Tile::Part(MachinePart {
                                        part_number,
                                        start: num_start,
                                        end: x - 1,
                                        row: y,
                                    }),
                                );
                            }
                            last_num_start_pos = None;
                        }
                    }
                    x += 1;
                } else {
                    if let Some(num_start) = last_num_start_pos {
                        let part_number: u32 = String::from_iter(line[num_start..x].chars())
                            .parse()
                            .expect("Fatal - failed to parse supposed part number");

                        for num_x in num_start..x {
                            coordinates.insert(
                                (num_x, y),
                                Tile::Part(MachinePart {
                                    part_number,
                                    start: num_start,
                                    end: x - 1,
                                    row: y,
                                }),
                            );
                        }
                    }

                    break;
                }
            }
        }

        Self { coordinates }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
        self.coordinates.get(&(x, y))
    }

    pub fn part_number_total(&self) -> u32 {
        let mut total = 0;
        let mut parts_uniq: HashSet<&MachinePart> = HashSet::new();

        for (_, tile) in self.coordinates.iter() {
            match tile {
                Tile::Part(part) => {
                    parts_uniq.insert(part);
                }
                _ => (),
            }
        }
        for p in parts_uniq {
            for (x, y) in p.bounding_box() {
                match self.get(x as usize, y as usize) {
                    Some(Tile::Symbol(c)) => {
                        println!("{}: {}", p.part_number, c);
                        total += p.part_number;
                        break;
                    }
                    _ => {}
                }
            }
        }
        total
    }
}

pub fn solution(lines: impl Iterator<Item = String>) -> u32 {
    Grid::from_lines(lines).part_number_total()
}
