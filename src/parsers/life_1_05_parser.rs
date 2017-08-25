use std::io::{BufRead, BufReader, Read};
use super::Parser;
use ::errors::{self, ResultExt};
use ::GameDescriptor;
use ::game_descriptor::DefaultGameDescriptor;

pub struct Life105Parser {}

impl Life105Parser {
	pub fn new() -> Self {
		Life105Parser{}
	}
}

impl Parser for Life105Parser {
	fn parse<T: Read>(&mut self, input: T) -> errors::Result<Box<GameDescriptor>> {
		let mut ret = DefaultGameDescriptor::new();
		for line in BufReader::new(input).lines() {
			let line = line.chain_err(|| "failed to read line")?;
			let line = line.trim();

			if line.starts_with("#R") {
				parse_rules(&line, &mut ret)?;
			}
		}

		Ok(Box::new(ret))
	}
}

fn parse_rules(
	line: &str,
	gd: &mut DefaultGameDescriptor
) -> errors::Result<()> {
	use regex::Regex;

	let regex = Regex::new("#R\\s*(\\d+)\\s*/\\s*(\\d+)\\s*").unwrap();
	let captures = regex.captures(line).unwrap();

	let survival = captures.get(1).unwrap().as_str();
	let birth = captures.get(2).unwrap().as_str();

	for char in survival.chars() {
		let digit = char.to_digit(10).unwrap() as u8;
		gd.add_survival(digit);
	}

	for char in birth.chars() {
		let digit = char.to_digit(10).unwrap() as u8;
		gd.add_birth(digit);
	}

	Ok(())
}
