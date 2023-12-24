use std::error::Error;

use crate::util::regex_or_panic;
use regex::Regex;

lazy_static! {
	static ref PART_REGEX: Regex = regex_or_panic(r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}");
}

const DIMENSIONS: &[&str; 4] = &["x", "m", "a", "s"];

 pub struct MachinePart {
	pub x: i64,
	pub m: i64,
	pub a: i64,
	pub s: i64
}

#[derive(Clone, Copy, Debug)]
pub enum PartAttribute { X, M, A, S }

impl MachinePart {
	pub fn from(s: &str) -> Result<Self, Box<dyn Error>> {
		let (_, [x, m, a, s]) = PART_REGEX
			.captures(s)
			.expect(&format!("Part regex failed to match line '{}'", s))
			.extract();

		Ok(Self {
			x: x.parse()?,
			m: m.parse()?,
			a: a.parse()?,
			s: s.parse()?
		})
	}
}

impl PartAttribute {
	pub fn from(c: &char) -> Self {
		match c {
			'x' => Self::X,
			'm' => Self::M,
			'a' => Self::A,
			's' => Self::S,
			_ => panic!("Attempted to parse invalid char '{}' as PartAttribute", c)
		}
	}
}