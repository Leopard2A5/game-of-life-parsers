/// Describes the initial state of a Game of Life.
pub trait GameDescriptor {
	/// Get the survival rules. Logically, this can contain up to 8 digits in the range [1,8].
	fn survival(&self) -> &[u8];

	/// Get the birth rules. Logically, this can contain up to 8 digits in the range [1,8].
	fn birth(&self) -> &[u8];

	/// Get the living cells. Coordinates can be negative, depending on the input file.
	fn live_cells(&self) -> &[Coord];

	/// Copy this `GameDescriptor`, adjusting for negative coordinates in the living cells.
	/// The new `GameDescriptor` will not contain negative coordinates.
	fn no_negative_coords(&self) -> Box<GameDescriptor>;
}

/// Represents the coordinates of cells.
#[derive(Debug, PartialEq)]
pub struct Coord {
	pub x: i16,
	pub y: i16,
}
