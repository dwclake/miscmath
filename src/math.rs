pub mod linear;

use std::ops::Range;

/* factorial function
   Calculates and returns the factorial of the integer entered */
pub fn factorial( fac_num: usize ) -> usize {
	if fac_num > 1 {
		fac_num * factorial( fac_num - 1 )
	} else {
		1
	}
}

/* fibonacci function
   Prints out the fibonacci sequence up to the number of terms entered */
pub fn fibonacci( ) {

}

/* map function
   Takes a value within a given range and converts it to an equivalent value in another range */
pub fn map(input: f64, in_rng: Range<f64>, out_rng: Range<f64> ) -> f64 {
	out_rng.start + ( ( out_rng.end - out_rng.start )
		          / ( in_rng.end - in_rng.start ) )
		          * ( input - in_rng.start )
}

#[cfg(test)]
mod math_tests {
	use super::*;
	
	#[test]
	fn it_works( ) {
		assert!( map(0.25, 0.0..1.0, 0.0..100.0 ) - 25.0 < 0.000000001 );
		assert!( map(0.50, 0.0..1.0, 0.0..100.0 ) - 50.0 < 0.000000001 );
		assert!( map(0.75, 0.0..1.0, 0.0..100.0 ) - 75.0 < 0.000000001 );
		assert_eq!( factorial(0), 1 );
		assert_eq!( factorial(1), 1 );
		assert_eq!( factorial(2), 2 );
		assert_eq!( factorial(3), 6 );
		assert_eq!( factorial(4), 24 );
		assert_eq!( factorial(5), 120 );
	}
}