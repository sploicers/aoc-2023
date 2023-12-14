use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

#[allow(unused)]
pub fn read_input_lines() -> impl Iterator<Item = String> {
    BufReader::new(get_input_reader())
        .lines()
        .flat_map(Result::ok)
}

#[allow(unused)]
pub fn get_input_reader() -> BufReader<Box<dyn Read>> {
    let reader: Box<dyn Read> = if let Some(filename) = filename_from_args() {
        Box::new(File::open(&filename).expect(&format!("No such file {}!", filename)))
    } else {
        Box::new(io::stdin().lock())
    };
    BufReader::new(reader)
}

fn filename_from_args() -> Option<String> {
    env::args().skip(1).collect::<Vec<_>>().first().cloned()
}
