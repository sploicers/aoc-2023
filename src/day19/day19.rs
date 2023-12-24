use std::io::{BufReader, Read};
use crate::day19::{processor::PartProccessor, part::MachinePart, workflow::Workflow};

pub fn part1(reader: &mut BufReader<Box<dyn Read>>) -> u64 {
	let mut buffer = String::new();
	reader
		.read_to_string(&mut buffer)
		.expect("Failed to read input");

	let (workflow_section, parts_section) = buffer
		.split_once("\n\n")
		.or_else(|| buffer.split_once("\r\n\r\n"))
		.expect("Input should be in two parts, separated by a blank line");

	let workflows: Vec<Workflow> = workflow_section
		.lines()
		.map(Workflow::from)
		.collect();

	println!("{:?}", workflows.first());

	let parts: Vec<MachinePart> = parts_section
		.lines()
		.flat_map(MachinePart::from)
		.collect();

	let part_processor = PartProccessor::new(workflows);

	0
}