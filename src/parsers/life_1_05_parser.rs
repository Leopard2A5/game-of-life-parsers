use std::io::{BufRead, BufReader, Read};
use super::Parser;
use ::errors::{self, Error, ErrorKind};
use ::GameDescriptor;
use ::default_game_descriptor::DefaultGameDescriptor;
use std::i16;

/// Parser for files in the life 1.05 format.
pub struct Life105Parser {}

impl Life105Parser {
	/// Construct a new instance.
	pub fn new() -> Self {
		Life105Parser{}
	}
}

impl Parser for Life105Parser {
	fn parse<'a>(&mut self, input: Box<Read + 'a>) -> errors::Result<Box<GameDescriptor>> {
		let mut ret = DefaultGameDescriptor::new();

		let mut offset = None;
		let mut line_in_block: i16 = 0;
		for (line_num, line) in BufReader::new(input).lines().enumerate() {
			let line_num = line_num + 1; // line numbers don't start at 0
			let line = line
				.map_err(|err| errors::ErrorKind::IOError(err.kind()))?;
			let line = line.trim();

			if line.starts_with("#R") {
				ret.clear_rules();
				parse_rules(line_num, &line, &mut ret)?;
			} else if line.starts_with("#Life") {
				check_file_format(line)?;
			} else if line.starts_with("#N") {
				ret.clear_rules();
				ret.add_survival(2);
				ret.add_survival(3);
				ret.add_birth(3);
			} else if line.starts_with("#P") {
				offset = Some(parse_offset(line_num, &line)?);
				line_in_block = 0;
			} else if let Some((ox, oy)) = offset {
				for (index, char) in line.chars().enumerate() {
					let index_with_offset = index as i32 + ox as i32;
					if index_with_offset > i16::MAX as i32 || index_with_offset < i16::MIN as i32 {
						bail!(ErrorKind::CoordinateOutOfRange(line_num));
					}
					match char {
						'*' => ret.add_live_cell(index as i16 + ox, line_in_block + oy),
						'.' => {},
						_ => bail!(ErrorKind::MalformedLine(line_num))
					}
				}
				line_in_block += 1;
			}
		}

		Ok(Box::new(ret))
	}
}

fn parse_rules(
	line_num: usize,
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
		bail!(ErrorKind::MalformedLine(line_num))
	}
}

fn parse_offset(
	line_num: usize,
	line: &str,
) -> errors::Result<(i16, i16)> {
	use regex::Regex;

	let regex = Regex::new("#P\\s*([+-]?\\d+)\\s*([+-]?\\d+)\\s*").expect("Invalid regex");
	if let Some(captures) = regex.captures(line) {
		let x = captures.get(1).unwrap().as_str();
		let y = captures.get(2).unwrap().as_str();

		let x = x.parse::<i16>()
			.map_err(|_| Error::from(ErrorKind::CoordinateOutOfRange(line_num)))?;
		let y = y.parse::<i16>()
			.map_err(|_| Error::from(ErrorKind::CoordinateOutOfRange(line_num)))?;

		Ok((x, y))
	} else {
		bail!(errors::ErrorKind::MalformedLine(line_num))
	}
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
		match parse_rules(5, "#R 23", &mut gd) {
			Err(Error(MalformedLine(5), _)) => {},
			Err(_) => panic!("Wrong error thrown!"),
			_ => panic!("No error thrown!")
		}
	}

	#[test]
	fn parse_rules_should_parse_custom_rules() {
		let mut gd = DefaultGameDescriptor::new();
		parse_rules(1, "#R24 / 1", &mut gd).unwrap();
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
			Err(Error(IOError(io::ErrorKind::BrokenPipe), _)) => {},
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
			Err(Error(InvalidFileFormat, _)) => {},
			Err(Error(x, _)) => panic!("Unexpected error {}", x),
			_ => panic!("No error thrown!"),

		}
	}

	#[test]
	fn should_raise_error_on_invalid_block_header() {
		let mut parser = Life105Parser::new();
		let input = "#P 0 a".as_bytes();
		match parser.parse(Box::new(input)) {
			Err(Error(MalformedLine(1), _)) => {},
			Err(_) => panic!("Wrong error thrown!"),
			_ => panic!("No error thrown!")
		}
	}

	#[test]
	fn parse_offset_should_handle_too_big_coords() {
		match parse_offset(1, "#P 0 32768") {
			Err(Error(CoordinateOutOfRange(1), _)) => {},
			Err(_) => panic!("Wrong error thrown!"),
			Ok(_) => panic!("No error thrown!")
		}
	}

	#[test]
	fn should_raise_error_on_unexpected_character_in_block() {
		let input = Box::new("#P -1 -1\n..*#".as_bytes());
		let mut parser = Life105Parser::new();
		match parser.parse(input) {
			Err(Error(MalformedLine(2), _)) => {},
			_ => panic!("Expected MalformedLine")
		}
	}

	#[test]
	fn should_handle_too_big_horizontal_coords() {
		let input = Box::new("#P 32768 0\n*".as_bytes());

		let mut parser = Life105Parser::new();
		let result = parser.parse(input);
		match result {
			Err(Error(CoordinateOutOfRange(1), _)) => {},
			_ => panic!("Expected CoordinateOutOfRange")
		}
	}
}
