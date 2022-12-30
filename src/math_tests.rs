#[cfg(test)]
mod math_tests {
	use crate::math;
	
	#[test]
	fn it_works( ) {
		assert!( math::map(0.25, 0.0..1.0, 0.0..100.0 ) - 25.0 < 0.000000001 );
		assert!( math::map(0.50, 0.0..1.0, 0.0..100.0 ) - 50.0 < 0.000000001 );
		assert!( math::map(0.75, 0.0..1.0, 0.0..100.0 ) - 75.0 < 0.000000001 );
		assert_eq!( math::factorial(0), 1 );
		assert_eq!( math::factorial(1), 1 );
		assert_eq!( math::factorial(2), 2 );
		assert_eq!( math::factorial(3), 6 );
		assert_eq!( math::factorial(4), 24 );
		assert_eq!( math::factorial(5), 120 );
	}
}