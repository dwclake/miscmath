pub mod vector;
pub mod matrix;

// Coordinate System Enum
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum System {
	CARTESIAN,
	POLAR,
}