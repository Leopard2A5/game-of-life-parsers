#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

pub mod errors {
	error_chain!();
}
mod game_descriptor;

pub use game_descriptor::GameDescriptor;

use errors::*;

#[cfg(test)]
mod test {
    #[test]
	fn test() {
		assert_eq!(true, false)
	}
}
