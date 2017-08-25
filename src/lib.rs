#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate regex;

pub mod errors {
	error_chain!{}
}
mod game_descriptor;
mod parsers;

pub use game_descriptor::GameDescriptor;
pub use game_descriptor::Coord;
pub use parsers::Parser;
pub use parsers::Life105Parser;
