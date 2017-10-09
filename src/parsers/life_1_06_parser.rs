use super::Parser;
use ::errors::{self, ResultExt};
use ::GameDescriptor;
use ::default_game_descriptor::DefaultGameDescriptor;
use std::io::{Read, BufRead, BufReader};
use regex::Regex;

/// Parser for files in the life 1.05 format.
pub struct Life106Parser {}

impl Life106Parser {
	/// Construct a new instance.
	pub fn new() -> Self {
		Life106Parser{}
	}
}

impl Parser for Life106Parser {
	fn parse<T: Read>(&mut self, input: T) -> errors::Result<Box<GameDescriptor>> {
		let reader = BufReader::new(input);
		let mut ret = DefaultGameDescriptor::new();
		let regex = Regex::new("(\\d+)\\s+(\\d+)")
			.expect("invalid regex!");

		for line in reader.lines() {
			let line = line.chain_err(|| "failed to read line")?;
			let line = line.trim();

			if regex.is_match(line) {
				let coords: Vec<i16> = line.split_whitespace()
					.map(|it| it.parse::<i16>().expect("Error parsing int!"))
					.collect();
				ret.add_live_cell(coords[0], coords[1]);
			}
		}

		Ok(Box::new(ret))
	}
}
