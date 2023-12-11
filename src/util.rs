use std::{env, io::{Read, self, BufReader, BufRead}, fs::File};

pub fn read_input_lines() -> impl Iterator<Item = String> {
	let reader: Box<dyn Read> = if let Some(filename) = filename_from_args() {
		Box::new(File::open(&filename).expect(&format!("No such file {}!", filename)))
	} else {
		Box::new(io::stdin().lock())
	};
	let buf = BufReader::new(reader);
	buf.lines().flat_map(Result::ok)
}

fn filename_from_args() -> Option<String> {
	env::args().skip(1).collect::<Vec<_>>().first().cloned()
}