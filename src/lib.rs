#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate regex;

pub mod errors {
	error_chain! {
		errors {
			InvalidRulesLine(line: String) {
				description("Invalid rules line"),
				display("Invalid rules definition in line '{}'", line),
			}
		}
	}
}
mod game_descriptor;
mod parsers;
mod stringreader;

pub use game_descriptor::GameDescriptor;
pub use game_descriptor::Coord;
pub use parsers::Parser;
pub use parsers::Life105Parser;
