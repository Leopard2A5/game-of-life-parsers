use std::io::{BufRead, BufReader, Read};
use super::Parser;
use ::errors::{self, ErrorKind};
use ::GameDescriptor;
use ::default_game_descriptor::DefaultGameDescriptor;

/// Parser for files in the life 1.05 format.
pub struct Life105Parser {}

impl Life105Parser {
	/// Construct a new instance.
	pub fn new() -> Self {
		Life105Parser{}
	}
}

impl Parser for Life105Parser {
	fn parse(&mut self, input: Box<Read>) -> errors::Result<Box<GameDescriptor>> {
		let mut ret = DefaultGameDescriptor::new();

		let mut offset = None;
		let mut line_in_block: i16 = 0;
		for line in BufReader::new(input).lines() {
			let line = line
				.map_err(|err| errors::ErrorKind::IOError(err.kind()))?;
			let line = line.trim();

			if line.starts_with("#R") {
				ret.clear_rules();
				parse_rules(&line, &mut ret)?;
			} else if line.starts_with("#Life") {
				check_file_format(line)?;
			} else if line.starts_with("#N") {
				ret.clear_rules();
				ret.add_survival(2);
				ret.add_survival(3);
				ret.add_birth(3);
			} else if line.starts_with("#P") {
				offset = Some(parse_offset(&line)?);
				line_in_block = 0;
			} else if let Some((ox, oy)) = offset {
				for (index, char) in line.chars().enumerate() {
					if char == '*' {
						ret.add_live_cell(index as i16 + ox, line_in_block + oy);
					}
				}
				line_in_block += 1;
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

	let regex = Regex::new("#R\\s*(\\d+)\\s*/\\s*(\\d+)\\s*")
		.expect("invalid regex!");

	if let Some(captures) = regex.captures(line) {
		let survival = captures.get(1)
			.expect("Failed to get regex group")
			.as_str();
		let birth = captures.get(2)
			.expect("Failed to get regex group")
			.as_str();

		for char in survival.chars() {
			let digit = char.to_digit(10)
				.expect("Failed to parse u8") as u8;
			gd.add_survival(digit);
		}

		for char in birth.chars() {
			let digit = char.to_digit(10)
				.expect("Failed to parse u8") as u8;
			gd.add_birth(digit);
		}

		Ok(())
	} else {
		bail!(ErrorKind::InvalidRulesLine(line.into()))
	}
}

fn parse_offset(line: &str) -> errors::Result<(i16, i16)> {
	use regex::Regex;

	let regex = Regex::new("#P\\s*([+-]?\\d+)\\s*([+-]?\\d+)\\s*").expect("Invalid regex");
	let captures = regex.captures(line).unwrap();

	let x = captures.get(1).unwrap().as_str();
	let y = captures.get(2).unwrap().as_str();

	let x = x.parse::<i16>().unwrap();
	let y = y.parse::<i16>().unwrap();

	Ok((x, y))
}

fn check_file_format(line: &str) -> errors::Result<()> {
	use regex::Regex;

	let regex = Regex::new("#Life\\s+1.05").expect("Invalid regex");
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
	use errors::Error;
	use errors::ErrorKind::*;
	use std::io;

	#[test]
	fn parse_rules_should_err() {
		let mut gd = DefaultGameDescriptor::new();
		if let Err(Error(InvalidRulesLine(x), _state)) = parse_rules("#R 23", &mut gd) {
			assert_eq!("#R 23", x);
		} else {
			assert!(false);
		}
	}

	#[test]
	fn parse_rules_should_parse_custom_rules() {
		let mut gd = DefaultGameDescriptor::new();
		parse_rules("#R24 / 1", &mut gd).unwrap();
		assert_eq!(&[2, 4], gd.survival());
		assert_eq!(&[1], gd.birth());
	}

	#[test]
	fn parser_should_understand_default_rules() {
		let mut parser = Life105Parser::new();
		let gd = parser.parse(Box::new("#N".as_bytes())).unwrap();
		assert_eq!(&[2, 3], gd.survival());
		assert_eq!(&[3], gd.birth());
	}

	#[test]
	fn should_correctly_handle_io_errors() {
		use self::io_test_util;

		let mut parser = Life105Parser::new();
		let reader = io_test_util::ErrReader::new(io::ErrorKind::BrokenPipe);

		match parser.parse(Box::new(reader)) {
			Err(errors::Error(errors::ErrorKind::IOError(io::ErrorKind::BrokenPipe), _)) => {},
			Err(_) => panic!("Wrong error returned!"),
			_ => panic!("No error returned!")
		}
	}

	#[test]
	fn should_throw_error_on_wrong_format_annotation() {
		let mut parser = Life105Parser::new();
		let input = Box::new("#Life 1.06\n0 0\n1 2".as_bytes());
		let res = parser.parse(input);
		match res {
			Err(errors::Error(errors::ErrorKind::InvalidFileFormat, _)) => {},
			Err(errors::Error(x, _)) => panic!("Unexpected error {}", x),
			_ => panic!("No error thrown!"),

		}
	}
}
