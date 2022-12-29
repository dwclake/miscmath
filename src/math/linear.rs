pub mod vector;
pub mod matrix;

/* Coordinate system enum */
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum System {
	CARTESIAN,
	POLAR,
}