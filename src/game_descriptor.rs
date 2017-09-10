pub trait GameDescriptor {
	fn survival(&self) -> &[u8];
	fn birth(&self) -> &[u8];
	fn live_cells(&self) -> &[Coord];
	fn no_negative_coords(&self) -> Box<GameDescriptor>;
}

#[derive(Debug, PartialEq)]
pub struct Coord {
	pub x: i16,
	pub y: i16,
}
