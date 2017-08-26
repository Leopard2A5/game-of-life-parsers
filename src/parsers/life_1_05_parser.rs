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

		let mut offset = None;
		for line in BufReader::new(input).lines() {
			let line = line.chain_err(|| "failed to read line")?;
			let line = line.trim();

			if line.starts_with("#R") {
				parse_rules(&line, &mut ret)?;
			} else if line.starts_with("#P") {
				offset = Some(parse_offset(&line)?);
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

fn parse_offset(line: &str) -> errors::Result<(i8, i8)> {
	use regex::Regex;

	let regex = Regex::new("#P\\s*([+-]?\\d+)\\s*([+-]?\\d+)\\s*").unwrap();
	let captures = regex.captures(line).unwrap();

	let x = captures.get(1).unwrap().as_str();
	let y = captures.get(2).unwrap().as_str();

	let x = x.parse::<i8>().unwrap();
	let y = y.parse::<i8>().unwrap();

	Ok((x, y))
}
