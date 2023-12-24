use std::collections::HashMap;

use super::{part::{MachinePart, PartAttribute}, workflow::Workflow};

pub struct PartProccessor {
	workflows: HashMap<String, Workflow>,
	min_vals: [i64; 4],
	max_vals: [i64; 4],
}

impl PartProccessor {
	pub fn new(workflows: Vec<Workflow>) -> Self {
		Self {
			workflows: workflows
				.into_iter()
				.map(|w| (w.name.clone(), w))
				.collect(),

			min_vals: [i64::MIN, i64::MIN, i64::MIN, i64::MIN],
			max_vals: [i64::MAX, i64::MAX, i64::MAX, i64::MAX]
		}
	}

	pub fn accepts(&self, part: &MachinePart) -> bool {
		[part.x, part.m, part.a, part.s]
			.iter()
			.enumerate().all(|(dimension, value)| {
				*value <= self.max_vals[dimension] && *value >= self.min_vals[dimension] })
	}

	fn expand(&mut self, dimension: &PartAttribute) {
		let d = *dimension as usize;
		self.min_vals[d] = self.min_vals[d] + 1;
	}

	fn contract(&mut self, dimension: &PartAttribute) {
		let d = *dimension as usize;
		self.min_vals[d] = self.min_vals[d] - 1;
	}
}
