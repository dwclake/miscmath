pub mod vector;
pub mod matrix;

/* Coordinate system enum */
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum System {
	CARTESIAN,
	POLAR,
}


pub struct UnitF {
	value: f64,
}

impl UnitF {
	
	/* new function
	   Generates a new instance of UnitF */
	pub fn new ( value: f64 ) -> UnitF {
		if (0.0..1.0).contains( &value ) {
			UnitF { value }
		} else {
			panic!( "Unit floats must be in the range [0.0, 1.0]")
		}
	}
	
	/* value function
	   Returns the value contain by self */
	pub fn value( &self ) -> f64 {
		self.value
	}
	
}