use super::Parser;
use ::errors::{self, Error, ErrorKind};
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

		for (line_num, line) in reader.lines().enumerate() {
			let line_num = line_num + 1;
			let line = line.
				map_err(|err| errors::ErrorKind::IOError(err.kind()))?;
			let line = line.trim();

			if line.starts_with("#Life") {
				check_file_format(line)?;
			} else if regex.is_match(line) {
				let coords: Vec<&str> = line.split_whitespace().collect();
				let x = coords[0].parse::<i16>()
					.map_err(|_| Error::from(ErrorKind::CoordinateOutOfRange(line_num)))?;
				let y = coords[1].parse::<i16>()
					.map_err(|_| Error::from(ErrorKind::CoordinateOutOfRange(line_num)))?;

				ret.add_live_cell(x, y);
			}
		}

		Ok(Box::new(ret))
	}
}

fn check_file_format(line: &str) -> errors::Result<()> {
	use regex::Regex;

	let regex = Regex::new("#Life\\s+1.06").expect("Invalid regex");
	if regex.is_match(line) {
		Ok(())
	} else {
		bail!(ErrorKind::InvalidFileFormat)
	}
}

#[cfg(test)]
mod test {
	extern crate io_test_util;

	use super::*;
	use std::io;
	use errors::Error;
	use errors::ErrorKind::*;

	#[test]
	fn should_correctly_handle_io_errors() {
		use self::io_test_util;

		let mut parser = Life106Parser::new();
		let reader = io_test_util::ErrReader::new(io::ErrorKind::NotFound);

		match parser.parse(Box::new(reader)) {
			Err(errors::Error(errors::ErrorKind::IOError(io::ErrorKind::NotFound), _)) => {},
			Err(_) => panic!("Wrong error returned!"),
			Ok(_) => panic!("No error returned!")
		}
	}

	#[test]
	fn should_throw_error_on_wrong_format_annotation() {
		let mut parser = Life106Parser::new();
		let input = Box::new("#Life 1.05\n#P -1 -1\n.*.".as_bytes());
		let res = parser.parse(input);
		match res {
			Err(errors::Error(errors::ErrorKind::InvalidFileFormat, _)) => {},
			Err(errors::Error(x, _)) => panic!("Unexpected error {}", x),
			_ => panic!("No error thrown!"),

		}
	}

	#[test]
	fn should_handle_too_big_coordinates() {
		let mut parser = Life106Parser::new();
		let input = Box::new("32768 0".as_bytes());
		match parser.parse(input) {
			Err(Error(CoordinateOutOfRange(1), _)) => {},
			Err(_) => panic!("Wrong error thrown!"),
			_ => panic!("No error thrown!")
		}
	}
}
