#![recursion_limit = "1024"]

//! Collection of parsers for Conway's game of life.
//!
//! Currently supported file formats:
//!
//! * Life 1.05
//!
//! ## Usage
//!
//! ```
//! extern crate game_of_life_parsers;
//! // use std::fs::File;
//! use game_of_life_parsers::{Parser, Life105Parser};
//!
//! fn main() {
//! 	// let file = File::open("file.life").unwrap();
//! 	let file = "#N\n#P 0 0\n..*".as_bytes();
//! 	let mut parser = Life105Parser::new();
//! 	let game_descriptor =  parser.parse(Box::new(file)).unwrap();
//! 	for live_cell in game_descriptor.live_cells() {
//! 		// iterate overe live cells
//!		}
//! }
//! ```

#[macro_use]
extern crate error_chain;
extern crate regex;

/// Error types generated by `error_chain`.
pub mod errors {
	use std::io;

	error_chain! {
		errors {
			IOError(error: io::ErrorKind)
			InvalidFileFormat
			MalformedLine(line: usize)
			CoordinateOutOfRange(line: usize)
		}
	}
}

mod game_descriptor;
mod default_game_descriptor;
mod parsers;

pub use game_descriptor::GameDescriptor;
pub use game_descriptor::Coord;
pub use parsers::Parser;
pub use parsers::Life105Parser;
pub use parsers::Life106Parser;
