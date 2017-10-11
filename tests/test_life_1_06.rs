extern crate game_of_life_parsers;

use std::fs::File;
use game_of_life_parsers::Coord;
use game_of_life_parsers::Parser;
use game_of_life_parsers::Life106Parser;

#[test]
fn parse_file() {
	let file = File::open("tests/life_1_06/test_1_06.life").unwrap();
	let mut parser = Life106Parser::new();

	let gd = parser.parse(Box::new(file)).unwrap();
	assert_eq!(&[
		Coord { x: 0, y: 0 },
		Coord { x: 1, y: 0 },
		Coord { x: 3, y: 5 }
	], gd.live_cells())
}
