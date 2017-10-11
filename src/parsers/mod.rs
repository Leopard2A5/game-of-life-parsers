mod life_1_05_parser;
mod life_1_06_parser;

use std::io::Read;
use ::errors;
use ::GameDescriptor;

pub use self::life_1_05_parser::Life105Parser;
pub use self::life_1_06_parser::Life106Parser;

/// Specifies a common interface for all game of life parser implementations.
pub trait Parser {
	/// Parse the given input, which can be any implementor of `std::io::Read`, e.g. `std::fs::File`.
	fn parse(&mut self, input: Box<Read>) -> errors::Result<Box<GameDescriptor>>;
}
