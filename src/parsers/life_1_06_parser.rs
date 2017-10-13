use super::Parser;
use ::errors;
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
	fn parse(&mut self, input: Box<Read>) -> errors::Result<Box<GameDescriptor>> {
		let reader = BufReader::new(input);
		let mut ret = DefaultGameDescriptor::new();
		let regex = Regex::new("(\\d+)\\s+(\\d+)")
			.expect("invalid regex!");

		for line in reader.lines() {
			let line = line.
				map_err(|err| errors::ErrorKind::IOError(err.kind()))?;
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

#[cfg(test)]
mod test {
	extern crate io_test_util;

	use super::*;
	use std::io;

	#[test]
	fn should_correctly_handle_io_errors() {
		use self::io_test_util;

		let mut parser = Life106Parser::new();
		let reader = io_test_util::ErrReader::new(io::ErrorKind::NotFound);

		if let Err(parser_error) = parser.parse(Box::new(reader)) {
			match *parser_error.kind() {
				errors::ErrorKind::IOError(inner_error) => {
					assert_eq!(io::ErrorKind::NotFound, inner_error);
				},
				_ => panic!("wrong error kind!")
			}
		} else {
			panic!("No error returned!")
		}
	}
}
