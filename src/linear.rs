pub mod vector;
pub mod matrix;

/// Coordinate system enum
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CoordSystem {
	CARTESIAN,
	POLAR,
}

/// Unitary float, a f64 which must be in the closed interval [0.0,1.0]
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
pub struct UnitF {
	value: f64,
}

impl UnitF {
	
	/// Generates a new instance of UnitF
	///
	/// # Examples
	///
	/// ```
	///
	///
	///
	/// ```
	///
	pub fn new ( value: f64 ) -> UnitF {
		if (0.0..1.0).contains( &value ) {
			UnitF { value }
		} else {
			UnitF { value: value.clamp( 0.0, 1.0 ) }
		}
	}
	
	/// Returns the value contain by self
	///
	/// # Examples
	///
	/// ```
	///
	///
	///
	/// ```
	///
	pub fn value( &self ) -> f64 {
		self.value
	}
	
}