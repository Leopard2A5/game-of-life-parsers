
pub trait GameDescriptor {
	fn survival(&self) -> &[u8];
	fn birth(&self) -> &[u8];
	fn live_cells(&self) -> &[Coord];
}

#[derive(Debug, PartialEq)]
pub struct Coord {
	pub x: u8,
	pub y: u8,
}

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

	pub fn add_live_cell(&mut self, x: u8, y: u8) {
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
}
