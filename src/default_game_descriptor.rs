use ::GameDescriptor;
use ::Coord;

pub struct DefaultGameDescriptor {
	survival: Vec<u8>,
	birth: Vec<u8>,
	live_cells: Vec<Coord>,
}

impl DefaultGameDescriptor {
	pub fn new() -> Self {
		DefaultGameDescriptor {
			survival: vec![],
			birth: vec![],
			live_cells: vec![],
		}
	}

	pub fn add_survival(&mut self, value: u8) {
		self.survival.push(value);
	}

	pub fn add_birth(&mut self, value: u8) {
		self.birth.push(value);
	}

	pub fn clear_rules(&mut self) {
		self.survival.clear();
		self.birth.clear();
	}

	pub fn add_live_cell(&mut self, x: i16, y: i16) {
		self.live_cells.push(Coord{ x: x, y: y });
	}
}

impl GameDescriptor for DefaultGameDescriptor {
	fn survival(&self) -> &[u8] {
		&self.survival
	}

	fn birth(&self) -> &[u8] {
		&self.birth
	}

	fn live_cells(&self) -> &[Coord] {
		&self.live_cells
	}

	fn no_negative_coords(&self) -> Box<GameDescriptor> {
		use std::cmp;

		let min_x = self.live_cells.iter()
			.map(|item| item.x)
			.min()
			.unwrap_or(0);
		let min_x = cmp::min(0, min_x);

		let min_y = self.live_cells.iter()
			.map(|item| item.y)
			.min()
			.unwrap_or(0);
		let min_y = cmp::min(0, min_y);

		let mut ret = DefaultGameDescriptor::new();
		for survival in &self.survival {
			ret.add_survival(*survival);
		}
		for birth in &self.birth {
			ret.add_birth(*birth);
		}

		for coord in &self.live_cells {
			ret.add_live_cell(
				coord.x - min_x,
				coord.y - min_y
			)
		}

		Box::new(ret)
	}
}

#[cfg(test)]
mod test {
	use ::Life105Parser;
	use ::Parser;
	use ::Coord;

	#[test]
	fn no_negative_coords_should_work() {
		let input = r#"
#N
#P 0 -2
.*.
#P -1 0
*..
"#;
		let mut parser = Life105Parser::new();
		let gd = parser.parse(Box::new(input.as_bytes()))
			.unwrap();
		assert_eq!(&[ Coord {x: 1, y: -2}, Coord {x: -1, y: 0} ], gd.live_cells());

		let gd = gd.no_negative_coords();
		assert_eq!(&[ Coord {x: 2, y: 0}, Coord {x: 0, y: 2} ], gd.live_cells());
	}
}
