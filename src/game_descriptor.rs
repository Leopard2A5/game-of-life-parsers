
pub trait GameDescriptor {
	fn survival(&self) -> &[u8];
	fn birth(&self) -> &[u8];
	fn live_cells(&self) -> &[Coord];
}

pub struct Coord {
	pub x: u8,
	pub y: u8,
}
