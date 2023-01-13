//! # Miscmath
//!
//! `miscmath` is a collection of general math functions
//! and linear algebra structures like vectors and matrices.

pub mod linear;
pub mod prelude;

use rand::{Rng, distributions::uniform::{SampleUniform, SampleRange}, thread_rng};
use std::ops::Range;

///
///
/// # Examples
///
/// ```
///
///
///
/// ```
///
pub fn constrain( val: &mut f32, rng: &Range<f32>) {
	*val = val.clamp( rng.start, rng.end );
}

/// Calculates and returns the factorial of the integer entered
///
/// # Examples
///
/// ```
/// let a = miscmath::factorial(5);
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
/// let a = miscmath::map(0.25, 0.0..1.0, 0.0..100.0 );
///
/// assert!( ( a - 25.0 ) < 0.000000001 );
/// ```
///
pub fn map(input: f32, in_rng: Range<f32>, out_rng: Range<f32> ) -> f32 {
	out_rng.start + ( ( out_rng.end - out_rng.start )
		/ ( in_rng.end - in_rng.start ) )
		* ( input - in_rng.start )
}

pub const DEFAULT_NOISE_SEED: usize = 0;

/// Generates a random number in the range provided
///
/// # Examples
///
/// ```
/// use miscmath::prelude::*;
///
/// let a = random( 0..10 );
/// let b = random( 0.0..10.0 );
///
/// assert!( ( a < 10 ) && ( a >= 0 ) );
/// assert!( (b < 10.0 ) && ( b > 0.0 ) );
/// ```
///
pub fn random<T: SampleUniform>( rng: Range<T> ) -> T
		                       where Range<T>: SampleRange<T>,{
	
	thread_rng().gen_range( rng )
}

/// Perlin Noise struct
///
/// # Examples
///
/// ```
/// use rusty_nature_of_code::prelude::*;
///
/// let perlin = Noise::new( DEFAULT_NOISE_SEED );
/// ```
///
#[derive(Debug)]
pub struct Perlin {
	pub seed: usize,
	permutation_table: [i16; 256]
}

impl Perlin {
	
	/// Initializes and returns a new Perlin object
	///
	/// # Examples
	///
	/// ```
	/// use miscmath::prelude::*;
	///
	/// let perlin = Perlin::new( DEFAULT_NOISE_SEED );
	///
	/// ```
	///
	pub fn new( seed: usize ) -> Perlin {
		let mut perlin = Perlin{
			seed,
			permutation_table: [-1; 256],
		};
		
		/* Sets each element to a number from 0 to 256, with no repeating values */
		for i in 0..256 {
			
			/* Finds a element that hasn't been set yet to assign to i */
			Perlin::permutation_gen( &mut perlin.permutation_table, i );
		}
		
		/* todo: implement the rest of the perlin noise algorithm, using the randomly generated permutation table to generate a array of interpolated
		         pseudo-random values */
		
		perlin
	}
	
	// Recursive: Takes i and adds it to the permutation table at a random index, as long as that element hasn't been set yet (is still -1).
	// If the element picked has been set already this function is called recursively until it picks a non-set element.
	fn permutation_gen(table: &mut [i16; 256], i: i16 ) {
		/* Pick a random int between 0 and 255 */
		let index = random( 0..256 );
		
		/* Base case: If a number has not been assigned to that element yet, assign it i */
		if table[index] == -1 {
			table[index] = i;
		} else if i <= 256 { /* Recursive case: If a number has been assigned to that element, try again */
			Perlin::permutation_gen( table, i );
		}
	}
	
	/// Gets a value between 0 and 1 using perlin Noise
	///
	/// # Examples
	///
	/// ```
	///
	/// ```
	///
	pub fn get<T>(_time: T) {
		todo!()
	}
	
}