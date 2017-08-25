mod life_1_05_parser;

use std::io::Read;
use ::errors;
use ::GameDescriptor;

pub use self::life_1_05_parser::Life105Parser;

pub trait Parser {
	fn parse<T: Read>(&mut self, input: T) -> errors::Result<Box<GameDescriptor>>;
}
