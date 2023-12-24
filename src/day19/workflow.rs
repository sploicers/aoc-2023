use crate::util::regex_or_panic;
use super::part::PartAttribute;
use regex::Regex;

lazy_static! {
	static ref WORKFLOW_REGEX: Regex = regex_or_panic(r"(?P<name>\w+)\{(?P<rules>.+)\}");
	static ref RULE_REGEX: Regex = regex_or_panic(r"?:\w[<|>]\d+:)?\w+");
}

#[derive(Debug)]
pub struct Workflow {
	pub name: String,
	rules: Vec<Rule>
}

#[derive(Debug)]
struct Rule {
	operand: Option<PartAttribute>,
	comparator: Option<Comparator>,
	threshold: Option<i64>,
	out: String
}

#[derive(Debug)]
enum Comparator { LT, GT }


impl Workflow {
	pub fn from(line: &str) -> Self {
		let sections = WORKFLOW_REGEX
			.captures(line)
			.expect(&format!("Fatal - input line did not match workflow regex: '{}'", line));

		let name = String::from(&sections["name"]);
		let rules = sections["rules"]
			.split(",")
			.map(Rule::from)
			.collect();

		Self { name, rules }
	}
}

impl Rule {
	pub fn from(s: &str) -> Self {
		if let Some((left, destination)) = s.split_once(":") {
			for c in ['<', '>'] {
				if let Some((dimension, threshold)) = left.split_once(c) {
					let operand = dimension.chars().next().expect("Dimension should be one of x,m,a,s");
					return Self {
						operand: Some(PartAttribute::from(&operand)),
						comparator: Some(Comparator::from(&c)),
						threshold: threshold.parse().ok(),
						out: String::from(destination)
					}
				}
			}
			panic!("Got rule in input which did not contain a '<' or '>'")

		} else {
			Self {
				operand: None,
				comparator: None,
				threshold: None,
				out: s.to_string()
			}
		}
	}
}

impl Comparator {
	pub fn from(c: &char) -> Self {
		match c {
			'<' => Self::LT,
			'>' => Self::GT,
			_ => panic!("Fatal - attempted to parse invalid input char '{}' as comparator", c.to_string())
		}
	}
}
