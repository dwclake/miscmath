//! # Math Lib
//!
//! `math_lib` is a collection of general math functions
//! and linear algebra structures like vectors and matrices.

pub mod linear;
pub use crate::linear::{CoordSystem, UnitF, vector,  matrix};

use std::ops::Range;

/// Calculates and returns the factorial of the integer entered
///
/// # Examples
///
/// ```
/// let a = math_lib::factorial(5);
///
/// assert_eq!( a, 120 );
/// ```
///
pub fn factorial( fac_num: usize ) -> usize {
	if fac_num > 1 {
		fac_num * factorial( fac_num - 1 )
	} else {
		1
	}
}

/// Prints out the fibonacci sequence up to the number of terms entered
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
pub fn fibonacci( ) {

}

/// Takes a value within a given range and converts it to an equivalent value in another range
///
/// # Examples
///
/// ```
/// let a = math_lib::map(0.25, 0.0..1.0, 0.0..100.0 );
///
/// assert!( ( a - 25.0 ) < 0.000000001 );
/// ```
///
pub fn map(input: f64, in_rng: Range<f64>, out_rng: Range<f64> ) -> f64 {
	out_rng.start + ( ( out_rng.end - out_rng.start )
		/ ( in_rng.end - in_rng.start ) )
		* ( input - in_rng.start )
}