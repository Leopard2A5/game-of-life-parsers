extern crate game_of_life_parsers;

use std::fs::File;
use game_of_life_parsers::GameDescriptor;
use game_of_life_parsers::Coord;
use game_of_life_parsers::Parser;
use game_of_life_parsers::Life105Parser;

#[test]
fn parse_file() {
	let file = File::open("tests/life_1_05/glider.life").unwrap();
	let mut parser = Life105Parser::new();

	let gd: Box<GameDescriptor> = parser.parse(Box::new(file)).unwrap();
	assert_eq!(&[2, 3], gd.survival());
	assert_eq!(&[1], gd.birth());
	assert_eq!(&[
		// block 1
		Coord { x:  0, y: -1 },
		Coord { x:  1, y: 0 },
		Coord { x: -1, y: 1 }, Coord { x: 0, y: 1 }, Coord { x: 1, y: 1 },
		// block 2
		Coord { x: 3, y: 2 }, Coord { x: 4, y: 3 }, Coord { x: 5, y: 4 }
	], gd.live_cells());
}
