use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn inverse(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

struct Grid<T: From<char>> {
    inner: Vec<T>,
    n_rows: usize,
    n_cols: usize,
}

impl<T: From<char>> Grid<T> {
    pub fn next(&self, i: usize, d: Direction) -> Option<(usize, &T)> {
        match d {
            Direction::Left => (i % self.n_cols != 0).then_some(i - 1),
            Direction::Right => ((i + 1) % self.n_cols != 0).then_some(i + 1),
            Direction::Up => (i >= self.n_cols).then_some(i - self.n_cols),
            Direction::Down => (i < self.n_rows - self.n_cols).then_some(i + self.n_cols),
        }
        .map(|i| (i, &self.inner[i]))
    }

    pub fn neighbours(&self, i: usize) -> Vec<(usize, &T)> {
        [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ]
        .iter()
        .flat_map(|d| self.next(i, *d))
        .collect()
    }
}

impl<T: From<char>> FromStr for Grid<T> {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let n_rows = lines.len();
        let n_cols = lines
            .first()
            .ok_or("grid can only be constructed from multi-line input")?
            .len();

        let data = lines
            .into_iter()
            .flat_map(|line| line.chars().map(|c| c.into()))
            .collect::<Vec<_>>();

        Ok(Self {
            inner: data,
            n_rows,
            n_cols,
        })
    }
}
