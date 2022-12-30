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